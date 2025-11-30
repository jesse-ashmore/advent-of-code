(ns day1
  (:require
   [clojure.string :as string]))

(def real (slurp "./resources/real"))

(def example "R5, L5, R5, R3")

(defn parse-input [str]
  (map #(vector (first %) (Integer/parseInt (clojure.string/join (rest %)) 10)) (clojure.string/split str #", ")))

(parse-input example)

(defn turn [heading next]
  (mod (case next
         \L (dec heading)
         \R (inc heading)) 4))

(defn take-step [heading step]
  (case heading
    0 [0 (* -1 step)]
    1 [step 0]
    2 [0 step]
    3 [(* -1 step)]))

(defn get-steps [instructions]
  (loop [[next & remaining] instructions
         heading 0
         steps []]
    (if (nil? next)
      steps
      (recur remaining
             (turn heading (first next))
             (conj steps (take-step (turn heading (first next)) 10))))))

(get-steps (parse-input example))

(defn part1 [input]
  (reduce + (map abs (reduce #(apply vector (map + %1 %2))
                             (get-steps (parse-input input))))))

(part1 example)