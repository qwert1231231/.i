;; Simple calculator example in i language

;; Define some numbers
(echo "Basic arithmetic operations:")

;; Addition
(echo "5 + 3 =")
(echo (add 5 3))

;; Subtraction  
(echo "10 - 4 =")
(echo (sub 10 4))

;; Multiplication
(echo "6 * 7 =")
(echo (mul 6 7))

;; Division
(echo "20 / 4 =")
(echo (div 20 4))

;; Combined operations
(echo "Complex expression: (5 + 3) * 2 =")
(echo (mul (add 5 3) 2))

;; Conditional example
(echo "Is 5 > 3?")
(echo (if (flow) "Yes, 5 is greater than 3" "No"))
