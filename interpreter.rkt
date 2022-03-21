#lang racket

(define (ite pred then else)
  (if pred then else))
(define false #f)
(define true #t)
(define (= left right)
  (equal? left right))
(define (allow resource_attribute_keys resource_attribute_values username user_group_membership)
(let ((a!1 (and (resource_attribute_keys "env")
                (or (= (resource_attribute_values "env") "public"))
                (resource_attribute_keys "owner")))
      (a!2 (and (resource_attribute_keys "env")
                (or (= (resource_attribute_values "env") "private"))
                (resource_attribute_keys "owner")
                (or (= (resource_attribute_values "owner") username)))))
  (or (ite (user_group_membership "access-public") a!1 false)
      (ite (user_group_membership "access-private") a!2 false))))
(define (resource_keys key)
  (or
   (= key "env")
   (= key "owner")))
(define (resource_values key)
  (cond
    [(= key "env") "public"]
    [(= key "owner") "ahelwer"]
    [else ""]))
(define username "ahelwer")
(define (group_membership group_name)
  (or
   (= group_name "access-public")
   (= group_name "access-private")))
(allow resource_keys resource_values username group_membership)
