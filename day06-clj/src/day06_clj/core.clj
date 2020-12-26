(ns day06-clj.core
  (:gen-class))

(require '[clojure.string :as string])

(defn count-yes [replies]
  (count (reduce (fn [acc x] (conj acc x)) #{} (string/split replies #""))))

(def load-input-anyone
  (with-open [rdr (clojure.java.io/reader "./data/input.txt")]
    (filter
      (fn [x] (not-empty x))
      (map
        (fn [xs] (string/join "" xs))
        (partition-by #(= "" %) (doall (line-seq rdr)))))))

(def sum-of-counts (apply + (map count-yes load-input-anyone)))

(def load-input-everyone
  (with-open [rdr (clojure.java.io/reader "./data/input.txt")]
    (filter
      (fn [xs] (not= '("") xs))
      (partition-by #(= "" %) (doall (line-seq rdr))))))

(defn group-set [xs]
  (map (fn [x] (set x)) xs))

(defn common-group-reply [xs]
  (apply clojure.set/intersection (group-set xs)))

(defn count-common-reply [xs]
  (count (common-group-reply xs)))

(def sum-of-counts-everyone (apply + (map count-common-reply load-input-everyone)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
