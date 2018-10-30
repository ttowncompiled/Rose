use token::TokenType;
use token::Token;
use ast::NodeType;
use ast::Node;
use ast::Statement;
use ast::Expression;
use ast::Program;
use ast::statement::binding::LetStatement;
use ast::statement::expression::ExpressionStatement;
use ast::statement::ret::ReturnStatement;
use ast::expression::prefix::PrefixExpression;
use ast::expression::infix::InfixExpression;
use ast::literal::ident::Identifier;
use ast::literal::blank::Blank;
use ast::literal::integer::IntegerLiteral;
use ast::literal::float::FloatLiteral;
use ast::literal::boolean::BooleanLiteral;
use ast::literal::inf::Inf;
use ast::literal::nan::NaN;
use ast::literal::nil::Nil;
use ast::literal::symbol::Symbol;
use object::ObjectType;
use object;

pub trait Evaluator {
    fn eval(&self, node: Box<dyn Node>) -> Option<Box<dyn object::Object>>;
}

pub struct RoseEvaluator {}

impl RoseEvaluator {
    pub fn new() -> RoseEvaluator {
        return RoseEvaluator{};
    }
}

impl RoseEvaluator {
    fn eval_program(&self, statements: &Vec<Box<dyn Statement>>) -> Option<Box<dyn object::Object>> {
        let mut result: Option<Box<dyn object::Object>> = None;
        for stmt in statements.iter() {
            match stmt.node_type() {
                NodeType::LET               => {
                    result = None;
                },
                NodeType::EXPRESSION        => {
                    if let Some(exp) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                        match exp.value {
                            Some(ref value) => result = self.eval_expression(&value),
                            None => result = None,
                        }
                    } else {
                        result = None;
                    }
                },
                NodeType::RETURN            => {
                    result = None;
                },
                _                           => result = None,
            }
        }
        return result;
    }

    fn eval_expression(&self, expression: &Box<dyn Expression>) -> Option<Box<dyn object::Object>> {
        match expression.node_type() {
            NodeType::PREFIX        => {
                if let Some(prefix) = expression.as_any().downcast_ref::<PrefixExpression>() {
                    let right: Option<Box<dyn object::Object>>;
                    match prefix.value {
                        Some(ref value) => right = self.eval_expression(&value),
                        None => return None,
                    }
                    return self.eval_prefix_expression(&prefix.token.ttype, right);
                } else {
                    return None;
                }
            },
            NodeType::INFIX         => {
                if let Some(infix) = expression.as_any().downcast_ref::<InfixExpression>() {
                    let left:Option<Box<dyn object::Object>>;
                    match infix.left {
                        Some(ref value) => left = self.eval_expression(&value),
                        None => return None,
                    }
                    let right: Option<Box<dyn object::Object>>;
                    match infix.right {
                        Some(ref value) => right = self.eval_expression(&value),
                        None => return None,
                    }
                    return self.eval_infix_expression(&infix.token.ttype, left, right);
                } else {
                    return None;
                }
            },
            NodeType::IDENT         => {
                return None;
            },
            NodeType::BLANK         => {
                if let Some(_) = expression.as_any().downcast_ref::<Blank>() {
                    return Some(Box::new(object::Blank::new()));
                } else {
                    return None;
                }
            },
            NodeType::INTEGER       => {
                if let Some(integer_literal) = expression.as_any().downcast_ref::<IntegerLiteral>() {
                    return Some(Box::new(object::Integer::new(integer_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::FLOAT         => {
                if let Some(float_literal) = expression.as_any().downcast_ref::<FloatLiteral>() {
                    return Some(Box::new(object::Float::new(float_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::BOOLEAN       => {
                if let Some(boolean_literal) = expression.as_any().downcast_ref::<BooleanLiteral>() {
                    return Some(Box::new(object::Boolean::new(boolean_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::INF           => {
                if let Some(_) = expression.as_any().downcast_ref::<Inf>() {
                    return Some(Box::new(object::Inf::new()));
                } else {
                    return None;
                }
            },
            NodeType::NAN           => {
                if let Some(_) = expression.as_any().downcast_ref::<NaN>() {
                    return Some(Box::new(object::NaN::new()));
                } else {
                    return None;
                }
            },
            NodeType::NIL           => {
                if let Some(_) = expression.as_any().downcast_ref::<Nil>() {
                    return Some(Box::new(object::Nil::new()));
                } else {
                    return None;
                }
            },
            NodeType::SYMBOL        => {
                return None;
            },
            _                       =>  return None,
        }
    }

    fn eval_prefix_expression(&self, op: &TokenType, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        return match op {
            TokenType::RW_NOT           => self.eval_prefix_op_not_expression(right),
            TokenType::OP_NOT           => self.eval_prefix_op_not_expression(right),
            TokenType::OP_ADD           => self.eval_prefix_op_add_expression(right),
            TokenType::OP_SUB           => self.eval_prefix_op_sub_expression(right),
            _                           => None,
        };
    }

    fn eval_infix_expression(&self, op: &TokenType, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        return match op {
            TokenType::RW_AND           => self.eval_infix_rw_and_expression(left, right),
            TokenType::RW_OR            => self.eval_infix_rw_or_expression(left, right),
            TokenType::RW_IS            => self.eval_infix_op_eq_expression(left, right),
            TokenType::OP_ADD           => self.eval_infix_op_add_expression(left, right),
            TokenType::OP_SUB           => self.eval_infix_op_sub_expression(left, right),
            TokenType::OP_MUL           => self.eval_infix_op_mul_expression(left, right),
            TokenType::OP_DIV           => self.eval_infix_op_div_expression(left, right),
            TokenType::OP_MOD           => self.eval_infix_op_mod_expression(left, right),
            TokenType::OP_POW           => self.eval_infix_op_pow_expression(left, right),
            TokenType::OP_EQ            => self.eval_infix_op_eq_expression(left, right),
            TokenType::OP_NEQ           => self.eval_infix_op_neq_expression(left, right),
            TokenType::OP_GT            => self.eval_infix_op_gt_expression(left, right),
            TokenType::OP_GTE           => self.eval_infix_op_gte_expression(left, right),
            TokenType::OP_LT            => self.eval_infix_op_lt_expression(left, right),
            TokenType::OP_LTE           => self.eval_infix_op_lte_expression(left, right),
            _                           => None,
        };
    }

    fn eval_prefix_op_not_expression(&self, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match right {
            Some(obj) => {
                match obj.object_type() {
                    ObjectType::BOOLEAN         => {
                        if let Some(bool_obj) = obj.as_any().downcast_ref::<object::Boolean>() {
                            return Some(Box::new(object::Boolean::new(! bool_obj.value)));
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_prefix_op_add_expression(&self, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match right {
            Some(obj) => {
                match obj.object_type() {
                    ObjectType::INTEGER         => {
                        if let Some(int_obj) = obj.as_any().downcast_ref::<object::Integer>() {
                            return Some(Box::new(object::Integer::new(int_obj.value)));
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_prefix_op_sub_expression(&self, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match right {
            Some(obj) => {
                match obj.object_type() {
                    ObjectType::INTEGER         => {
                        if let Some(int_obj) = obj.as_any().downcast_ref::<object::Integer>() {
                            return Some(Box::new(object::Integer::new(- int_obj.value)));
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_rw_and_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::BOOLEAN          => {
                        if let Some(left_bool_obj) = left_obj.as_any().downcast_ref::<object::Boolean>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::BOOLEAN         => {
                                            if let Some(right_bool_obj) = right_obj.as_any().downcast_ref::<object::Boolean>() {
                                                return Some(Box::new(object::Boolean::new(left_bool_obj.value && right_bool_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_rw_or_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::BOOLEAN          => {
                        if let Some(left_bool_obj) = left_obj.as_any().downcast_ref::<object::Boolean>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::BOOLEAN         => {
                                            if let Some(right_bool_obj) = right_obj.as_any().downcast_ref::<object::Boolean>() {
                                                return Some(Box::new(object::Boolean::new(left_bool_obj.value || right_bool_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_add_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Integer::new(left_int_obj.value + right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_sub_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Integer::new(left_int_obj.value - right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_mul_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Integer::new(left_int_obj.value * right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_div_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Integer::new(left_int_obj.value / right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_mod_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Integer::new(left_int_obj.value % right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_pow_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Integer::new(left_int_obj.value.pow( right_int_obj.value as u32))));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_eq_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Boolean::new(left_int_obj.value == right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    ObjectType::BOOLEAN          => {
                        if let Some(left_bool_obj) = left_obj.as_any().downcast_ref::<object::Boolean>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::BOOLEAN         => {
                                            if let Some(right_bool_obj) = right_obj.as_any().downcast_ref::<object::Boolean>() {
                                                return Some(Box::new(object::Boolean::new(left_bool_obj.value == right_bool_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_neq_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Boolean::new(left_int_obj.value != right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    ObjectType::BOOLEAN          => {
                        if let Some(left_bool_obj) = left_obj.as_any().downcast_ref::<object::Boolean>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::BOOLEAN         => {
                                            if let Some(right_bool_obj) = right_obj.as_any().downcast_ref::<object::Boolean>() {
                                                return Some(Box::new(object::Boolean::new(left_bool_obj.value != right_bool_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_gt_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Boolean::new(left_int_obj.value > right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_gte_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Boolean::new(left_int_obj.value >= right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_lt_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Boolean::new(left_int_obj.value < right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }

    fn eval_infix_op_lte_expression(&self, left: Option<Box<dyn object::Object>>, right: Option<Box<dyn object::Object>>) -> Option<Box<dyn object::Object>> {
        match left {
            Some(left_obj) => {
                match left_obj.object_type() {
                    ObjectType::INTEGER          => {
                        if let Some(left_int_obj) = left_obj.as_any().downcast_ref::<object::Integer>() {
                            match right {
                                Some(right_obj) => {
                                    match right_obj.object_type() {
                                        ObjectType::INTEGER         => {
                                            if let Some(right_int_obj) = right_obj.as_any().downcast_ref::<object::Integer>() {
                                                return Some(Box::new(object::Boolean::new(left_int_obj.value <= right_int_obj.value)));
                                            } else {
                                                return None;
                                            }
                                        },
                                        _                           => return None,
                                    }
                                },
                                None => return None,
                            }
                        } else {
                            return None;
                        }
                    },
                    _                           => return None,
                }
            },
            None => return None,
        }
    }
}

impl Evaluator for RoseEvaluator {
    fn eval(&self, node: Box<dyn Node>) -> Option<Box<dyn object::Object>> {
        match node.node_type() {
            NodeType::PROGRAM       => {
                if let Some(program) = node.as_any().downcast_ref::<Program>() {
                    return self.eval_program(&program.statements);
                } else {
                    return None;
                }
            },
            NodeType::LET           => {
                return None;
            },
            NodeType::EXPRESSION    => {
                if let Some(expression) = node.as_any().downcast_ref::<ExpressionStatement>() {
                    match expression.value {
                        Some(ref value) => return self.eval_expression(&value),
                        None => return None,
                    }
                } else {
                    return None;
                }
            },
            NodeType::RETURN        => {
                return None;
            },
            NodeType::PREFIX        => {
                if let Some(prefix) = node.as_any().downcast_ref::<PrefixExpression>() {
                    let right: Option<Box<dyn object::Object>>;
                    match prefix.value {
                        Some(ref value) => right = self.eval_expression(&value),
                        None => return None,
                    }
                    return self.eval_prefix_expression(&prefix.token.ttype, right);
                } else {
                    return None;
                }
            },
            NodeType::INFIX         => {
                return None;
            },
            NodeType::IDENT         => {
                return None;
            },
            NodeType::BLANK         => {
                if let Some(_) = node.as_any().downcast_ref::<Blank>() {
                    return Some(Box::new(object::Blank::new()));
                } else {
                    return None;
                }
            },
            NodeType::INTEGER       => {
                if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
                    return Some(Box::new(object::Integer::new(integer_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::FLOAT         => {
                if let Some(float_literal) = node.as_any().downcast_ref::<FloatLiteral>() {
                    return Some(Box::new(object::Float::new(float_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::BOOLEAN       => {
                if let Some(boolean_literal) = node.as_any().downcast_ref::<BooleanLiteral>() {
                    return Some(Box::new(object::Boolean::new(boolean_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::INF           => {
                if let Some(_) = node.as_any().downcast_ref::<Inf>() {
                    return Some(Box::new(object::Inf::new()));
                } else {
                    return None;
                }
            },
            NodeType::NAN           => {
                if let Some(_) = node.as_any().downcast_ref::<NaN>() {
                    return Some(Box::new(object::NaN::new()));
                } else {
                    return None;
                }
            },
            NodeType::NIL           => {
                if let Some(_) = node.as_any().downcast_ref::<Nil>() {
                    return Some(Box::new(object::Nil::new()));
                } else {
                    return None;
                }
            },
            NodeType::SYMBOL        => {
                return None;
            },
            _                       =>  return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use token::Token;
    use object::ObjectType;

    #[test]
    fn test_eval_integer_expression() {
        let evaluator: RoseEvaluator = RoseEvaluator::new();
        let node: Box<dyn Node> = Box::new(IntegerLiteral::new(
            Token::new(TokenType::LIT_INT, "5".to_string(), 1, 1),
            5
        ));
        match evaluator.eval(node) {
            Some(obj) => {
                assert_eq!(ObjectType::INTEGER, obj.object_type());
                assert_eq!("5", obj.inspect());
            },
            None => assert!(false, "expect {}, got None", 5)
        }
        let node: Box<dyn Node> = Box::new(IntegerLiteral::new(
            Token::new(TokenType::LIT_INT, "10".to_string(), 1, 1),
            10
        ));
        match evaluator.eval(node) {
            Some(obj) => {
                assert_eq!(ObjectType::INTEGER, obj.object_type());
                assert_eq!("10", obj.inspect());
            },
            None => assert!(false, "expect {}, got None", 10)
        }
    }
}
