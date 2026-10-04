function mergeSort(arr: number[]): number[] {
    if (arr.length <= 1) return arr;
    const mid = Math.floor(arr.length / 2);
    const left = mergeSort(arr.slice(0, mid));
    const right = mergeSort(arr.slice(mid));
    const res: number[] = [];
    let i = 0, j = 0;
    while (i < left.length && j < right.length) {
        if (left[i] <= right[j]) res.push(left[i++]);
        else res.push(right[j++]);
    }
    return res.concat(left.slice(i)).concat(right.slice(j));
}

const sorted = mergeSort([12, 11, 13, 5, 6, 7]);
if (JSON.stringify(sorted) !== JSON.stringify([5, 6, 7, 11, 12, 13])) throw new Error("Failed");
console.log("[PASS] TypeScript Merge Sort passed!");
