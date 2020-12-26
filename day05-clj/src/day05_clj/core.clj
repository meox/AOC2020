(ns day05-clj.core
  (:gen-class))

(require '[clojure.string :as str])

(defn lower [range]
  (let [[l u] range
        d     (quot (- u l) 2)]
    [l (+ l d)]))

(defn upper [range]
  (let [[l u] range
        d (quot (- u l) 2)]
    [(+ l d 1) u]))

(defn engine [xs map-fn range]
  (if
    (empty? xs)
    (first range)
    (let [y (first xs)
          ys (rest xs)
          new-range (get map-fn y)]
      (engine ys map-fn (new-range range)))))

(defn row [input] (engine (str/split input #"") {"F" lower "B" upper} [0 127]))
(defn cols [input] (engine (str/split input #"") {"L" lower "R" upper} [0 7]))

(defn calc-rowcol [input]
  [(row (subs input 0 7)) (cols (subs input 7))])

(defn seat-id [seat]
  (+ (* (first seat) 8) (second seat)))

(def load-input
  (with-open [rdr (clojure.java.io/reader "./data/input.txt")]
    (doall (line-seq rdr))))

(defn highest-seat-id [rows]
  (apply max (map (fn [x] (seat-id (calc-rowcol x))) rows)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
