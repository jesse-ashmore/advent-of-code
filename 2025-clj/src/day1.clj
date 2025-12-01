(ns day1
  (:require
   [clojure.string :as string]))

(def real (slurp "./resources/1/real"))
(def example (slurp "./resources/1/example"))

(defn parse-input [str]
  (->> (clojure.string/split-lines str)
       (map #(case (first %)
               \L (* -1 (Integer/parseInt (subs % 1)))
               \R (Integer/parseInt (subs % 1))))))

(parse-input example)

(defn part-one [inst]
  (loop [[next & remaining] inst
         dial 50
         clicks 0]
    (if (nil? next)
      clicks
      (let [new_dial (mod (+ next dial) 100)
            new_clicks (+ clicks (if (zero? new_dial) 1 0))]
        (recur remaining new_dial new_clicks)))))

(defn includes-extra-click [dial rot]
  (let [applied (+ dial (rem rot 100))]
    (if (zero? dial)
      false
      (if (< rot 0)
        (< applied 0)
        (> applied 100)))))

;; (is-partial-rot 1 -5)
;; (rem -120 100)

(defn part-two [inst]
  (loop [[next & remaining] inst
         dial 50
         clicks 0]
    (if (nil? next)
      clicks
      (let [new_dial (mod (+ next dial) 100)
            new_clicks (+ clicks
                          ; Clicks from full rotations
                          (int (/ (abs next) 100))
                          ; Clicks from remaining partial rotations
                          (if (and (not (zero? new_dial)) (includes-extra-click dial next)) 1 0)
                          ; If we land on 0, that's an extra click
                          (if (zero? new_dial) 1 0))]
        ;; (println (str new_dial " " new_clicks))
        (recur remaining new_dial new_clicks)))))

(part-one (parse-input real))
(part-two (parse-input real))