(defun fact (n) (if (zerop n) 1 (* n (fact (- n 1)))))

(defun fact (n)
  (if (zerop n)
    1
    (* n (fact (- n 1)))
  )
)

(tim (prog1 nil (fact 100000)))

