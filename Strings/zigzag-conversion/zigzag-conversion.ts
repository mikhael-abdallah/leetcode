// https://leetcode.com/problems/zigzag-conversion/description/
function convert(s: string, numRows: number): string {
  if (numRows === 1) {
    return s;
  }
  let res = Array.from({ length: numRows }).fill("") as string[];
  let direction = 1;
  let actualColumn = 0;
  for (let i = 0; i < s.length; i++) {
    res[actualColumn] = res[actualColumn].concat(s[i]);

    actualColumn = actualColumn + direction;

    if (actualColumn < 0 || actualColumn >= numRows) {
      direction *= -1;
      actualColumn += 2 * direction;
    }
  }
  return res.join("");
}
