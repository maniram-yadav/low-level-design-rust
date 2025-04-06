## Pizza Store System LLD Functional Documentation
###   Core Functionality
1. Menu Item Management
Base Pizza Types

Create and price different pizza bases (Regular, Thin Crust, Deep Dish)

Each base has a distinct name and price

Toppings Management

Add/configure various toppings (Cheese, Pepperoni, Vegetables, etc.)

Each topping has a name and individual price

Drinks Management

Add/configure drink options (Soda, Beer, Water)

Each drink has a name and price

###   2. Pizza Customization
Pizza Builder

Create pizzas by selecting base and multiple toppings

Name custom pizza combinations

Automatic price calculation based on components

Component Pricing

Dynamic price calculation for any pizza configuration

Sum of base price + all selected toppings

###   3. Order Management
Order Composition

Add multiple pizzas to an order

Include drinks and other items

Flexible combination of different item types

Order Pricing

Calculate subtotal of all items

Apply multiple promotional deals

Calculate final total after discounts

###   4. Promotional Deals System
Buy One Get One Free (Pizzas)

Discount equals price of cheapest pizza when ordering 2+

Free Drink with Pizza

Discount equals price of cheapest drink when ordering pizza

Most Expensive Topping Free

Discount equals price of most expensive topping on each pizza

Extensible Deal Framework

Easy to add new deal types without modifying core logic

Deals can be combined in single order

###   5. Store Configuration
Multiple Store Support

Each store maintains its own pricing

Independent menu configurations

Store-specific promotional deals

Pricing Management

Set different prices for same items across stores

Modify prices without code changes

###   6. Reporting
Order Breakdown

Detailed item listing with individual prices

Applied deals with descriptions

Clear subtotal/discount/total display

##  Technical Features
###   1. Design Patterns Implemented
Composite Pattern: Unified treatment of individual items and composed pizzas

Strategy Pattern: Flexible discount calculation through interchangeable deal algorithms

Builder Pattern: Step-by-step pizza construction with validation

Factory Pattern: Centralized item creation with store-specific pricing

###   2. Type Safety
Compile-time checking of all item types

Safe downcasting for deal application logic

Proper error handling for missing items

###   3. Ownership Management
Clear ownership semantics for all menu items

Proper cloning where needed

Lifetime management for store references

###   4. Extensibility Points
New Item Types: Can add without modifying existing code

New Deals: Implement Deal trait for additional promotions

New Stores: Create with independent configurations

## Usage Scenarios Covered
Basic Order

Single pizza with multiple toppings

Drink added separately

No deals applied

Deal Combinations

Multiple pizzas with BOGO deal

Pizza + drink with free drink deal

Most expensive topping discount

Store Variations

Different pricing at different locations

Location-specific promotions

Complex Orders

Multiple pizzas with different configurations

Various drinks

Multiple deals applied simultaneously

Edge Cases

Empty orders

Orders without pizzas

Orders with only drinks

Attempting to apply invalid deals

Business Rules Enforced
Pizza Requirements

Must have a name

Must have a base

May have 0-N toppings

Deal Validation

Only store-approved deals can be applied

Deal prerequisites checked (e.g., must have pizza for free drink deal)

Pricing Rules

All prices positive

Discounts cannot make total negative

Component prices sum correctly