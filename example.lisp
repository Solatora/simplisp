(define 'content (trim (file-read "example_input.txt")))

(define 'elves (split "\r\n\r\n" content))

(define 'elves (map 
    (function '(elf) 
        '(quoted (map 
            (function '(value) '(as-number value)) 
            (split "\r\n" elf)))) 
    elves))

(define 'elves (map (function '(elf) '(apply '+ elf)) elves))

(define 'result (fold 
    (function '(acc curr) 
        '(if (> acc curr) 
            acc 
            curr)) 
    0 
    elves))
    
result

