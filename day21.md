Each allergen is found in exactly one ingredient.
Each ingredient contains zero or one allergen.
Allergens aren't always marked.
Even if an allergen isn't listed, the ingredient that contains that allergen could still be present

4             mxmxvkd      sbzzf sqjhc     (contains fish)
1       kfcds mxmxvkd nhms       sqjhc     (contains dairy, fish)
2 fvjkl       mxmxvkd      sbzzf       trh (contains dairy)
3 fvjkl                          sqjhc     (contains soy)

1   K M N    SQ   (contains dairy, fish)
2 F   M   SB    T (contains dairy)
3 F          SQ   (contains soy)
4     M   SB SQ   (contains fish)
  2 1 3 1  2  3 1

SB
 1: does not contain dairy, fish
 3: does not contain soy

K
 2: cannot contain dairy
 3: cannot contain soy
 4: cannot contain fish
 If you're only in rule 1, you cannot be an allergen

If you're not in a rule that produces an allergen, you can't contain that allergen

N
 (same logic as K, can't be an allergen)

T
  3: does not contain soy
  4: does not contain fish
  1: does not contain dairy

4&1: M SQ contains fish
1&2: M    contains dairy

 F = fvjkl
 K = kfcds
 M = mxmxvkd
 N = nhms
SB = sbzzf
SQ = sqjhc
 T = trh

kfcds, nhms, sbzzf, or trh cannot contain an allergen
K      N     SB        T
-> F, M, SQ can
allergens = fish, dairy, soy

Sample: 7 ingredients, 3 allergens
Input: 200 ingredients, 8 allergens

Sample after filtering out safe ingredients:

1        mxmxvkd sqjhc -> {"dairy", "fish"}
2 fvjkl  mxmxvkd       -> {"dairy"}
3 fvjkl          sqjhc -> {"soy"}
4        mxmxvkd sqjhc -> {"fish"}

sample: 3 / 3 ingredients / allergens
 input: 8 / 8 ingredients / allergens

1/4 => F is not dairy or fish => F is soy
2/S => S is not dairy => S is soy or fish

bmrmhm,rkkrx,snhrpv,vflms,bqkndvb,zmb,qzkjrtl,bqtvr

mxmxvkd contains dairy.
sqjhc contains fish.
fvjkl contains soy.

mapping: [("dairy", "fvjkl"), ("fish", "mxmxvkd"), ("soy", "sqjhc")]
answer: fvjkl,mxmxvkd,sqjhc