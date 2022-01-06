(ns aoc21-clojure.d08
  (:require [clojure.string :as str])
  (:use [clojure.set])
  (:use [clojure.test]))

(defn get-tables [m]
  (let [real-code-to-digit { "cf" 1 "acf" 7 "bcdf" 4 "acdeg" 2 "acdfg" 3 "abdfg" 5 "abcefg" 0 "abdefg" 6 "abcdfg" 9 "abcdefg" 8 }
        map-code #(str/join (sort (vals (select-keys %1 %2))))
        t1 (into {} (map (fn [[real-code digit]] [(map-code m real-code) digit])
                         real-code-to-digit))
        m2 (rename-keys m {\c \f, \f \c})
        t2 (into {} (map (fn [[real-code digit]] [(map-code m2 real-code) digit])
                         real-code-to-digit))]
    [t1 t2]))

(defn decode [[left right]]
  (let [left-codes (str/split left #" ")
        right-codes (str/split right #" ")
        l (group-by count left-codes)
        l2 (get-in l [2 0])
        l3 (get-in l [3 0])
        l4 (get-in l [4 0])
        l3-l2 (first (difference (set l3) (set l2)))
        m { \c (get l2 0) \f (get l2 1) \a l3-l2 }
        bd (difference (set l4) #{(m \c) (m \f)})
        bd-freq (map-invert (frequencies (filter #(contains? bd %) (str/join (l 5)))))
        m (assoc m \b (bd-freq 1) \d (bd-freq 3))
        abcdf (vals (select-keys m "abcdef"))
        l6-red (map #(difference (set %) abcdf) (l 6))
        eg-freq (map-invert (frequencies (str/join (map str/join l6-red))))
        m (assoc m \e (eg-freq 2) \g (eg-freq 3))
        [t1 t2] (get-tables m)
        right-codes-sorted (map #(str/join (sort %)) right-codes)
        digits (map #(or (t1 %) (t2 %)) right-codes-sorted)
        value (Integer/parseInt (str/join (map str digits)))]
    value))

(defn parse-input [line]
  (let [parts (str/split line #"\|")]
    (map str/trim parts)))

(testing
    (is (=
         (let [examples ["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
                         "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
                         "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
                         "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
                         "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
                         "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
                         "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
                         "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
                         "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
                         "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"]]
           (->> examples
                (map parse-input)
                (map decode)
                (reduce +)))
         61229)))

(defn part2 []
  (->> "../data/d08.txt"
      slurp
      str/split-lines
      (map parse-input)
      (map decode)
      (reduce +)))

(prn (part2)) ; 975706
