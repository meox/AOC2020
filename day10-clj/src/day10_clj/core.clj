(ns day10-clj.core
  (:gen-class))

(def mini [16 10 15 5 1 11 7 19 6 12 4])
(def large [28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3])

(defn load-input [fname]
  (with-open [rdr (clojure.java.io/reader fname)]
    (doall (line-seq rdr))))

(def input
  (map (fn [x] (Integer/parseInt x)) (load-input "./data/input.txt")))

(defn setup-list [xs]
  (let [input (vec (sort xs))
       h      (last input)]
    (cons 0 (conj input (+ 3 h)))))

(defn jolt-gaps [xs]
  (->> (map vector xs (rest xs))
    (map (fn [x] (- (second x) (first x))))))

(defn count-gaps [gaps]
  (->> gaps
    (group-by identity)
    (map (fn [e] [(first e) (count (second e))]))))

(defn part1 [xs]
  (apply * (map (fn [e] (second e)) xs)))

(defn is-in? [x xs]
  (if
    (empty? xs)
    false
    (let [h  (first xs)
          rs (rest xs)]
      (if (= h x) true (is-in? x rs)))))

(defn sub-tree [xs v]
  (drop (.indexOf xs v) xs))

(defn possible-sub-trees [xs vs]
  (map (fn [v] (sub-tree xs v)) vs))

(defn jolt-tree [xs]
  (if
    (= (count xs) 1)
    1
    (let [h  (first xs)
          rs (rest xs)
          ps (list (+ 1 h) (+ 2 h) (+ 3 h))
          vs (filter (fn [x] (is-in? x rs)) ps)
          ts (possible-sub-trees rs vs)]
      (apply + (map jolt-tree ts)))))


(def not-nil? (complement nil?))

; (dfs-jolt-tree (setup-list mini) 0 {})
(defn dfs-jolt-tree [xs acc visited]
  (if
    (= (count xs) 1)
    [(inc acc) visited]
    (let [h         (first xs)
          rs        (rest xs)
          nexts     (list (+ 1 h) (+ 2 h) (+ 3 h))
          childrens (filter (fn [x] (is-in? x rs)) nexts)]
      (reduce
        (fn [state x]
          (let [[nacc c-visited] state
                val              (c-visited x)]
            (if
              (not-nil? val)
              [(+ nacc val) c-visited]
              (let [tree   (sub-tree rs x)
                    result (dfs-jolt-tree tree nacc c-visited)
                    facc   (+ nacc (first result))
                    fv     (assoc (second result) x (first result))]
                [facc fv]))))
        [acc visited]
        childrens))))

(defn part2 [xs]
  (let [jlist (setup-list xs)]
    (first (dfs-jolt-tree jlist 0 {}))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
