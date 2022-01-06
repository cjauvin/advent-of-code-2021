from collections import defaultdict

S = """be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"""

def part1():
    r = 0
    #for line in s.split('\n'):
    for line in open('/home/christian/gh/advent-of-code-2021/data/d08.txt'):
        if not line.strip():
            continue
        a = line.split('|')[1]
        b = a.strip().split()
        c = filter(lambda x: len(x) in [2, 4, 3, 7], b)
        r += len(list(c))
    return r

def get_tables(m):
    # compute two tables because "1" is ambiguous
    real_code_to_digit = {
        'cf': 1,
        'acf': 7,
        'bcdf': 4,
        'acdeg': 2,
        'acdfg': 3,
        'abdfg': 5,
        'abcefg': 0,
        'abdefg': 6,
        'abcdfg': 9,
        'abcdefg': 8,
    }
    t1 = {}
    for real_code, digit in real_code_to_digit.items():
        mapped_sorted_code = ''.join(sorted(m[s] for s in real_code))
        t1[mapped_sorted_code] = digit
    t2 = {}
    m['c'], m['f'] = m['f'], m['c']
    for real_code, digit in real_code_to_digit.items():
        mapped_sorted_code = ''.join(sorted(m[s] for s in real_code))
        t2[mapped_sorted_code] = digit
    return t1, t2

def part2():

    total = 0
    # for line in open('/home/christian/gh/advent-of-code-2021/data/d08.txt'):
    for line in S.split('\n'):
        if not line.strip():
            continue
        parts = line.split('|')
        left_codes = parts[0].strip().split()
        l = defaultdict(list)  # lengths
        for code in left_codes:
            l[len(code)].append(set(code))

        m = {}  # seg mapping (real -> given)
        l2 = iter(l[2][0])
        m['c'] = next(l2)
        m['f'] = next(l2)
        m['a'] = next(iter(l[3][0] - l[2][0]))

        # b and d segments
        bd = l[4][0] - {m['c'], m['f']}

        bd_freq = defaultdict(int)  # seg -> freq
        for s5 in l[5]:
            for s in s5 & bd:
                bd_freq[s] += 1

        for s, freq in bd_freq.items():
            assert freq in [1, 3]
            if freq == 1:
                m['b'] = s
            else:
                m['d'] = s

        abcdf = {m[s] for s in 'abcdf'}

        eg_freq = defaultdict(int)
        for s6 in l[6]:
            for s in s6 - abcdf:
                eg_freq[s] += 1

        for s, freq in eg_freq.items():
            assert freq in [2, 3]
            if freq == 2:
                m['e'] = s
            else:
                m['g'] = s

        assert set(m.keys()) == set('abcdefg')
        assert set(m.values()) == set('abcdefg')

        t1, t2 = get_tables(m)

        res = []
        for code in parts[1].strip().split():
            k = ''.join(sorted(code))
            res.append(t1[k] if k in t1 else t2[k])

        v = int(''.join(map(str, res)))
        total += v

    return total

print(part2())
print()
