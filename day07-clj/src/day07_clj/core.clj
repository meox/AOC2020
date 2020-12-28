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
    {:name name, :contain bags}))

(defn parse-rules [rules]
  (reduce
    (fn [acc x] (assoc acc (x :name) (x :contain)))
    {}
    (map parse-rule rules)))

(defn load-input [fname]
  (with-open [rdr (clojure.java.io/reader fname)]
    (doall (line-seq rdr))))

(def rules (parse-rules (load-input "./data/input.txt")))

(defn contained [vs] (map (fn [m] (m :name)) vs))

(defn store-into [acc x keys]
  (reduce
    (fn [acc k]
      (let [cs (acc k)]
        (assoc acc k (conj cs x))))
    acc
    keys))

(defn invert-index [rules]
  (reduce
    (fn [acc x]
      (let [name (first x)
            vs  (contained (second x))]
        (store-into acc name vs)))
    {}
    rules))

(defn enum-bags [index bag]
  (let [containers (index bag)]
    (clojure.set/union
      (set containers)
      (set
        (mapcat
          (fn [x] (enum-bags index x))
          containers)))))

(defn count-bags [index bag] (count (enum-bags index bag)))

(defn count-bags-inside [bag rules]
  (let [vs (rules bag)]
    (if
      (empty? vs)
      0
      (apply +
        (map
          (fn [x] (let [name (x :name)
                        q    (x :quantity)]
                  (* q (inc (count-bags-inside name rules)))))
          vs)))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
