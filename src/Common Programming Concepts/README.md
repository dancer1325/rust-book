# Variables and Mutability
* By default, immutable
* `mut variableName`
  * make a variable mutable in the value
    * Note: ⚠️type can NOT be changed⚠️
* shadowing
  * depends on the scope
  * ≠ `mut`
    * shadow variable — can be still — immutable
## Constants
* := values / immutable ALWAYS
* ≠ immutable variables
  * 👁️NOT allowed `mut constantName` 👁️
  * can be declared in any scope
  * type must be declared ALWAYS == can NOT be inferred
  * = constant expression
    * == NOT possible to be computed at runtime
* `const constantName: Type`
* CAPITAL_SEPARATED_WITH_UNDERSCORE
  * convention to declare a constant


# Notes
* All samples code are '/listings'