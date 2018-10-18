---
title: Single Responsibility Principle
permalink: /docs/single-responsibility-principle/
---

Violation of the Single Responsibility Principle

{% highlight rust %}
    class SpaceStation
        mut supplies : {} of String => UInt
        mut fuel : UInt

        pub SpaceStation(mut self : Self)
            self.supplies = {}
            self.fuel = 0
        end

        pub def run_sensors
            @puts "----- Sensor Action -----"
            @puts "Running sensors!"
        end

        pub def load_supplies(self: &mut Self, type: String, quantity: UInt)
            @puts "----- Supply Action -----"
            @puts "Loading {quantity} units of {type} in the supply hold."
            self.supplies[type] += quantity
        end

        pub def use_supplies(self: &mut Self, type: String, quantity: UInt)
            @puts "----- Supply Action -----"
            if type in self.supplies and self.supplies[type] >= quantity
                @puts "Using {quantity} of {type} from the supply hold."
                self.supplies[type] -= quantity
            else
                @puts "Supply Error: Insufficient {type} in the supply hold."
            end
        end

        pub def report_supplies
            @puts "----- Supply Report -----"
            if self.supplies.keys.length > 0
                self.supplies.each do |type : String, quantity: UInt|
                    @puts "{type} available : {quantity} units."
                end
            else
                @puts "Supply hold is empty."
            end
        end

        pub def load_fuel(self: &mut Self, quantity: UInt)
            @puts "----- Fuel Action -----"
            @puts "Loading {quantity} units of fuel in the tank."
            self.fuel += quantity
        end

        pub def report_fuel
            @puts "----- Fuel Report -----"
            @puts "{self.fuel} units of fuel available."
        end

        pub def activate_thrusters(self: &mut Self)
            @puts "----- Thruster Action -----"
            if self.fuel >= 10
                @puts "Thrusting action successful."
                self.fuel -= 10
            else
                @puts "Thruster Error: Insufficient fuel available."
            end
        end
    end

    # based on work by Severin Perez in
    # "Writing Flexible Code with the Single Responsbility Principle"
{% endhighlight %}

Instituting the Single Responsibility Principle

{% highlight rust %}
    class SpaceStation
        has sensors : Sensors
        has mut supply_hold : SupplyHold
        has mut fuel_tank : FuelTank
        pub thrusters : Thrusters

        pub SpaceStation(mut self: Self)
            self.sensors = new Sensors
            self.supply_hold = new SupplyHold
            self.fuel_tank = new FuelTank
            self.thrusters = new Thrusters
        end
    end

    trait Sensors
        pub def run_sensors
            @puts "----- Sensor Action -----"
            @puts "Running sensors!"
        end
    end

    class SupplyHold
        mut supplies : {} of String => UInt

        pub SupplyHold(mut self: Self)
            self.supplies = {}
        end

        pub get get_supplies => self.supplies

        pub def load_supplies(self: &mut Self, type: String, quantity: UInt)
            @puts "----- Supply Action -----"
            @puts "Loading {quantity} units of {type} in the supply hold."
            self.supplies[type] += quantity
        end

        pub def use_supplies(self: &mut Self, type: String, quantity: UInt)
            @puts "----- Supply Action -----"
            if type in self.supplies and self.supplies[type] >= quantity
                @puts "Using {quantity} of {type} from the supply hold."
                self.supplies[type] -= quantity
            else
                @puts "Supply Error: Insufficient {type} in the supply hold."
            end
        end

        pub def report_supplies
            @puts "----- Supply Report -----"
            if self.supplies.keys.length > 0
                self.supplies.each do |type : String, quantity: UInt|
                    @puts "{type} available : {quantity} units."
                end
            else
                @puts "Supply hold is empty."
            end
        end
    end

    class FuelTank
        mut fuel : UInt

        pub FuelTank(mut self : Self)
            self.fuel = 0
        end

        pub get get_fuel_levels => self.fuel

        pub def load_fuel(self: &mut Self, quantity: UInt)
            @puts "----- Fuel Action -----"
            @puts "Loading {quantity} units of fuel in the tank."
            self.fuel += quantity
        end

        pub def use_fuel(self: &mut Self, quantity: UInt)
            @puts "----- Fuel Action -----"
            if self.fuel >= quantity
                @puts "Using {quantity} units of fuel from the tank."
                self.fuel -= quantity
            else
                @puts "Fuel Error: Insufficient fuel available."
            end
        end

        pub def report_fuel
            @puts "----- Fuel Report -----"
            @puts "{self.fuel} units of fuel available."
        end
    end

    trait Thrusters
        FUEL_PER_THRUST := 10

        pub def activate_thrusters(self : &Self, fuel_tank: &mut FuelTank)
            @puts "----- Thruster Action -----"
            if fuel_tank.get_fuel_levels >= self.FUEL_PER_THRUST
                @puts "Thrusting action successful."
                fuel_tank.use_fuel(self.FUEL_PER_THRUST)
            else
                @puts "Thruster Error: Insufficient fuel available."
            end
        end
    end

    # based on work by Severin Perez in
    # "Writing Flexible Code with the Single Responsbility Principle"
{% endhighlight %}

Extending the functionality of the program

{% highlight rust %}
    class SpaceStation
        has sensors: Sensors
        has mut supply_hold : SupplyHold
        pub supply_reporter : SupplyReporter
        has mut fuel_tank : FuelTank
        pub fuel_reporter : FuelReporter
        pub thrusters : Thrusters

        pub SpaceStation(mut self: Self)
            self.sensors = new Sensors
            self.supply_hold = new SupplyHold
            self.supply_reporter = new SupplyReporter
            self.fuel_tank = new FuelTank
            self.fuel_reporter = new FuelReporter
            self.thrusters = new Thrusters
        end
    end

    trait Sensors
        pub def run_sensors
            @puts "----- Sensor Action -----"
            @puts "Running sensors!"
        end
    end

    class SupplyHold
        mut supplies : {} of String => UInt

        pub SupplyHold(mut self: Self)
            self.supplies = {}
        end

        pub get get_supplies => self.supplies

        pub def load_supplies(self: &mut Self, type: String, quantity: UInt)
            @puts "----- Supply Action -----"
            @puts "Loading {quantity} units of {type} in the supply hold."
            self.supplies[type] += quantity
        end

        pub def use_supplies(self: &mut Self, type: String, quantity: UInt)
            @puts "----- Supply Action -----"
            if type in self.supplies and self.supplies[type] >= quantity
                @puts "Using {quantity} of {type} from the supply hold."
                self.supplies[type] -= quantity
            else
                @puts "Supply Error: Insufficient {type} in the supply hold."
            end
        end
    end

    class FuelTank
        mut fuel : UInt

        pub FuelTank(mut self : Self)
            self.fuel = 0
        end

        pub get get_fuel_levels => self.fuel

        pub def load_fuel(self: &mut Self, quantity: UInt)
            @puts "----- Fuel Action -----"
            @puts "Loading {quantity} units of fuel in the tank."
            self.fuel += quantity
        end

        pub def use_fuel(self: &mut Self, quantity: UInt)
            @puts "----- Fuel Action -----"
            if self.fuel >= quantity
                @puts "Using {quantity} units of fuel from the tank."
                self.fuel -= quantity
            else
                @puts "Fuel Error: Insufficient fuel available."
            end
        end
    end

    trait Thrusters
        FUEL_PER_THRUST := 10

        pub def activate_thrusters(self : &Self, fuel_tank: &mut FuelTank)
            @puts "----- Thruster Action -----"
            if fuel_tank.get_fuel_levels >= self.FUEL_PER_THRUST
                @puts "Thrusting action successful."
                fuel_tank.use_fuel(self.FUEL_PER_THRUST)
            else
                @puts "Thruster Error: Insufficient fuel available."
            end
        end
    end

    trait Reporter
        pub def report(self: &Self, type: String)
            @puts "----- {type.capitalize} Report -----"
        end
    end

    trait FuelReporter ext Reporter
        FUEL_TYPE := "fuel"

        pub def report(self: &Self, item: &FuelTank)
            self.super.report(self.FUEL_TYPE)
            @puts "{item.get_fuel_levels} units of fuel available."
        end
    end

    trait SupplyReport ext Reporter
        SUPPLY_TYPE := "supply"

        pub def report(self: &Self, item: &SupplyHold)
            self.super.report(self.SUPPLY_TYPE)
            if item.get_supplies.keys.length > 0
                item.get_supplies.each do |type : String, quantity: UInt|
                    @puts "{type} available : {quantity} units."
                end
            else
                @puts "Supply hold is empty."
            end
        end
    end

    iss = new SpaceStation

    iss.run_sensors
    #=> "----- Sensor Action ----"
    #=> "Running sensors!"

    iss.use_supplies("parts", 2)
    #=> "----- Supply Action -----"
    #=> "Supply Error: Insufficient parts in the supply hold."

    iss.load_supplies("parts", 10)
    #=> "----- Supply Action -----"
    #=> "Loading 10 units of parts in the supply hold."

    iss.use_supplies("parts", 2)
    #=> "----- Supply Action -----"
    #=> "Using 2 of parts from the supply hold."

    iss.supply_reporter.report
    #=> "----- Supply Report -----"
    #=> "parts available : 8 units."

    iss.thrusters.activate_thrusters
    #=> "----- Thruster Action -----"
    #=> "Thruster Error: Insufficient fuel available."

    iss.load_fuel(100)
    #=> "----- Fuel Action -----"
    #=> "Loading 100 units of fuel in the tank."

    iss.thrusters.activate_thrusters
    #=> "----- Thruster Action -----"
    #=> "Thrusting action successful."
    #=> "----- Fuel Action -----"
    #=> "Using 10 units of fuel from the tank."


    iss.fuel_reporter.report
    #=> "----- Fuel Report -----"
    #=> "90 units of fuel available."

    # based on work by Severin Perez in
    # "Writing Flexible Code with the Single Responsbility Principle"
{% endhighlight %}

