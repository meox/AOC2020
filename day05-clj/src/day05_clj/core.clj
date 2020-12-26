(ns day05-clj.core
  (:gen-class))

(defn front [range]
  (let [[l u] range
        d     (quot (- u l) 2)]
    [l (+ l d)]))

(defn back [range]
  (let [[l u] range
        d (quot (- u l) 2)]
    [(+ l d 1) u]))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
