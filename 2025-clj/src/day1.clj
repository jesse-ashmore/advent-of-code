(ns day1
  (:require
   [clojure.string :as string]))

(def real (slurp "./resources/1/real"))
(def example (slurp "./resources/1/example"))

(defn parse-input [str]
  (->> (clojure.string/split-lines str)
       (map #(case (first %)
               \L (* -1 (Integer/parseInt (subs % 1)))
               \R (Integer/parseInt (subs % 1)))              ; Remove first character
            )))

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

(defn is-partial-rot [dial rot]
  (let [applied (+ dial rot)]
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
                          (int (/ (abs next) 100))
                          (if (and (not (zero? new_dial)) (is-partial-rot dial (rem next 100))) 1 0)
                          (if (zero? new_dial) 1 0))]
        ;; (println (str new_dial " " new_clicks))
        (recur remaining new_dial new_clicks)))))

(part-one (parse-input real))
(part-two (parse-input real))