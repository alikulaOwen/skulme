function quickSort(arr: number[]): number[] {
    if (arr.length <= 1) return arr;
    const pivot = arr[Math.floor(arr.length / 2)];
    const left = arr.filter(x => x < pivot);
    const mid = arr.filter(x => x === pivot);
    const right = arr.filter(x => x > pivot);
    return quickSort(left).concat(mid).concat(quickSort(right));
}

if (JSON.stringify(quickSort([10, 7, 8, 9, 1, 5])) !== JSON.stringify([1, 5, 7, 8, 9, 10])) throw new Error("Failed");
console.log("[PASS] TypeScript Quick Sort passed!");
