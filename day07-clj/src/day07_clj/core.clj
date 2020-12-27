(ns day07-clj.core
  (:gen-class))

(require '[clojure.string :as string])

(defn clean-str [s] (string/trim (string/replace s #" bags?\.?" "")))
(defn parse-item-notempty [s]
  (let [tks      (string/split s #" " 2)
        quantity (Integer/parseInt (first tks))
        name     (second tks)]
    {:name name, :quantity quantity}))

(defn parse-item [s]
  (let [cleaned (clean-str s)]
    (if
      (= cleaned "no other")
      nil
      (parse-item-notempty cleaned))))

(defn parse-contains [s]
  (let [tks (string/split s #",")]
    (filter #(not (nil? %)) (map parse-item tks))))

(defn parse-rule [s]
  (let [parts (string/split s #" bags contain ")
        bags  (parse-contains (second parts))
        name  (first parts)]
    [name bags]))

(defn parse-rules [rules]
  (reduce
    (fn [acc x] (assoc acc (first x) (second x)))
    {}
    (map parse-rule rules)))

(defn load-input [fname]
  (with-open [rdr (clojure.java.io/reader fname)]
    (doall (line-seq rdr))))

(def rules (parse-rules (load-input "./data/mini.txt")))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
