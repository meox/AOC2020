(ns day09-clj.core
  (:gen-class)
  :require [clojure.math.combinatorics :as combo]))

(defn gen-pairs [xs] (doall (combo/combinations xs 2)))

(defn is-in? [x xs]
  (if
    (empty? xs)
    false
    (let [h  (first xs)
          rs (rest xs)]
      (if (= h x) true (is-in? x rs)))))

(defn valid-next? [x xs]
  (is-in?
    x
    (map
      (fn [p] (apply + p))
      (gen-pairs xs))))

(defn find-invalid [xs preamble-len]
  (let [cs (partition (inc preamble-len) 1 xs)]
    (first
      (drop-while
        (fn [e] (true? (second e)))
        (map
          (fn [es]
            (let [h (last es)]
              [h (valid-next? h es)]))
          cs)))))

(def mini '(35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576))

(defn load-input [fname]
  (with-open [rdr (clojure.java.io/reader fname)]
    (doall (line-seq rdr))))

(def input
  (map (fn [x] (biginteger x)) (load-input "./data/input.txt")))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
