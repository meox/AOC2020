(ns day03-clj.core
  (:gen-class))

(require '[defun.core :refer [defun]])

(def example-input '(
  "..##......."
  "#...#...#.."
  ".#....#..#."
  "..#.#...#.#"
  ".#...##..#."
  "..#.##....."
  ".#.#.#....#"
  ".#........#"
  "#.##...#..."
  "#...##....#"
  ".#..#...#.#"
  ))

(defn convert-row [str]
  (map (fn [x] (if (= x \.) 0 1)) str))

(defn convert-input [xs] (map convert-row xs))

(def load-input-file
  (with-open [rdr (clojure.java.io/reader "./data/input.txt")]
    (convert-input (doall (line-seq rdr)))))

(defn roll2 [xs orig]
  (if
    (empty? xs)
    (roll2 orig orig)
    (lazy-seq (cons (first xs) (roll2 (rest xs) orig)))))
(defn roll [xs] (roll2 xs xs))

(defn select-tree [map-tree col row]
  (nth (roll (nth map-tree row)) col))

(defn is-tree? [map-tree col row]
  (= (select-tree map-tree col row) 1))

(defun count-tree
  ([map-tree slope] (count-tree map-tree slope (count map-tree) [0 0] 0))
  ([map-tree slope n coord num-tree]
    (let [[col row] coord
          [dx dy] slope
          next-col (+ col dx)
          next-row (+ row dy)]
      (if (<= n 0)
        num-tree
        (count-tree
        map-tree
        slope
        (- n dy)
        [next-col next-row]
        (if (is-tree? map-tree col row) (inc num-tree) num-tree))))))


(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
