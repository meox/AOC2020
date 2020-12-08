(ns day02-clj.core
  (:require
    [clojure.string :as str])
  (:gen-class))

; 1-3 a: abcde
; 1-3 b: cdefg
; 2-9 c: ccccccccc

(def input (list 
                "1-3 a: abcde"
                "1-3 b: cdefg"
                "2-9 c: ccccccccc"))

(def load-input
  (with-open [rdr (clojure.java.io/reader "./data/input.txt")]
    (doall (line-seq rdr))))

(defn parse-policy
  [policy]
  (let [tks   (str/split policy #" ")
        range (str/split (first tks) #"-")
        ch    (last tks)]
    {:min (Integer/parseInt (first range))
     :max (Integer/parseInt (last range))
     :ch ch}))

(defn parse-rule
  [rule]
  (let [tks    (str/split rule #": ")
        policy (parse-policy (first tks))
        pass   (last tks)]
    [policy pass]))

(defn is-pass-valid [policy pass]
  (let [pass-hist (frequencies (str/split pass #""))
        {ch :ch
         min :min
         max :max} policy
        n (get pass-hist ch)]
    (and
      (not (nil? n))
      (>= n min)
      (<= n max))))

(defn is-pass-valid-2 [policy pass]
  (let [{ch :ch
         min :min
         max :max} policy
        a          (str (get pass (- min 1)))
        b          (str (get pass (- max 1)))]
    (and
      (or
        (= a ch)
        (= b ch))
      (not= a b))))

(defn count-valid-pass [predicate]
  (count
    (filter
      (fn [line]
        (let [rule   (parse-rule line)
             [policy pass] rule]
          (predicate policy pass)))
      load-input)))

(defn -main
  "day-02"
  [& args]
  (println
    (count-valid-pass is-pass-valid)
    (count-valid-pass is-pass-valid-2)))
