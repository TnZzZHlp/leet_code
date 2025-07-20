function groupAnagrams(strs: string[]): string[][] {
    let map = new Map();
    for (const str of strs) {
        let key = str.split("").sort().join("");

        if (!map.has(key)) {
            map.set(key, [str]);
        } else {
            map.get(key).push(str);
        }
    }

    return [...map.values()];
}

console.log(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));
console.log(groupAnagrams([""]));
console.log(groupAnagrams(["a"]));
console.log(groupAnagrams(["", ""]));
