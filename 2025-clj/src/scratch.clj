(ns scratch)

(or (= 0 1) false)

(fn  rev [seq]
  (loop [[next & remaining] seq
         r []]
    (if (nil? next) r (recur remaining (cons next r)))))

;; (rev '([1 2] [3 4] [5 6]))

(apply (fn odd [seq]
         (loop [[next & remaining] seq
                coll []
                iter 0]
           (if (nil? next)
             coll
             (if (= (mod iter 2) 1)
               (let coll coll)
               (recur remaining (conj coll next) (inc iter)))) [1 2 3 4]))

(defn flat [[next & rest]]
  (if (nil? next)
    nil
    (if (sequential? next)
      (flat (concat next rest))
      (cons next (flat rest)))))

(flat [[1 2] 2])
       
(filter (fn isCaps [x] (let [asStr (.toString x)]
                         (= (.toUpperCase asStr) asStr)) "aBcD"))
Character/isUpperCase