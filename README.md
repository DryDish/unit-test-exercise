# unit_test_exercise
School assignment for unit tests



---

## Part one 

Create  a  backend  application  that  performs conversions  across  several  measuring  object-oriented programming and according to the following specification:

* Length
    * Conversion between the Metric and Imperial systems
    * It only covers centimeters and inches, respectively
    * Length class. Methods:
        * Constructor 
            * `@param The numeric measure to convert with up to two decimals`
            * `@param The system of said measure (Metric or Imperial)`
        * convert()
            * It implements an `if` (if the system is Metric then ... otherwise ...)
            * `@return The value of the conversion with up to two decimals`
    * Weight
        * Conversion between the Metric and Imperial systems
        * It only covers kilograms and pounds, respectively
        * Free class implementation
    * Temperature
        * Conversion between the Celsius, Fahrenheit, and Kelvin scales
        * Temperature class. Methods:
            * Constructor
                * `@param The numeric measure to convert with up to two decimals`
                * `@param The temperature scale of said measure`
            * Convert()
                * It implements a switch with the 6 possible conversions
                `(C to F, C to K, F to C, F to K, K to C, K to F)`
                * Each switch calls a method that performs the specific conversion
                * `@param The destination temperature scale`
                * `@return The value of the conversion with up to two decimals`

## Part two

* Design and write unit tests for all the classes
    * Or maybe not (if you think that, in some case,they do not bring value)
* Design a comprehensive set of test cases
    * Apply black-box techniques 
    * Look for extreme cases
* Write beautiful, efficient, maintainable unit tests
    * Use parameterized tests, data providers or similar
* Upload your code to Fronter

_Assignments are authored by Arturo Mora-Rioja (amri@kea.dk) - 2022_

---