(erase-messages)

(message
 (format "dotted pair notation: %s"
         (let ((items '(a . b))) items)))

(message
 (format "(cons 'a 'b): %s"
         (let ((items (cons 'a 'b))) items)))

(message
 (format "(list 'a 'b \"c\"): %s"
         (let ((items (list 'a 'b "c"))) items)))


(message
 (format "%s"
         (buffers-names-in-current-frame)

))
