use std::result::Result;

use ramp::Int;

/// Returns a factor `p` of `n` if `p - 1` is b-powersmooth
/// ```
/// use pollard_p_minus_one::factor;
///
/// let n = 299;
/// let b = 4;
/// println!("Found factor {}", factor(n, b).unwrap());
/// ```
pub fn factor<T: Into<Int>>(n: T, b: usize) -> Result<Int, String> {
    stage_one(&n.into(), b)
}

fn stage_one(n: &Int, b: usize) -> Result<Int, String> {
    let mut a = Int::from(2);
    let sieve = primal::Sieve::new(b);
    let mut i = 1;
    for p in sieve.primes_from(0).take_while(|x| *x <= b) {
        let p = Int::from(p);
        let mut pp = p.clone();
        // TODO: Can we reduce to only one pow_mod?
        while pp <= b {
            a = a.pow_mod(&p, &n);
            pp *= &p;
        }

        // If we've guessed a value for `b` that's much larger than the optimal GCD, we can do GCD
        // checks along the way to avoid waiting until the end to compute it
        if i % 10000 == 0 {
            let res = (&a - Int::one()).gcd(&n);
            if &res == n {
                return Err(format!(
                    "No factors found, b is too large. Last prime checked was {}",
                    p
                ));
            } else if res != 1 {
                return Ok(res);
            }
        }
        i += 1;
    }
    let res = (&a - Int::one()).gcd(&n);
    if res == 1 {
        Err("No factors found, b is too small".into())
    } else if &res == n {
        Err("No factors found, b is too large".into())
    } else {
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_test() {
        let n = 299; // 299 = 13 * 23
        let b = 4;
        assert_eq!(factor(n, b), Ok(Int::from(13)));
    }

    // This test is slow, only run it on release builds
    #[cfg(not(debug_assertions))]
    #[test]
    fn large_test() {
        use std::str::FromStr;

        let n = "117911911918369790919720975317727997203707046601051351826721271874783853131751609098765731046029447933261636972813116742407130875299141787357683469241777658197786984438598371402518887397817933105113479511486553793259993969901597308956327518540097915428630057666285736301168703069467190961313648991101481264839233016190317090643055741947798931130103688323127786904532647620687468212608218188863199140879962414171562373218840608222219318179409091939363954623199073612619845014342755459692145512627408086947448095714986225640631183289881812344329746336161760611271830871349452975544249067994440287113890425199826188613612186351107673851664103933072047542732683713562402036060948844572684056632664596773055801173780758873602639193789440537314216611521223926376339024027089951362455743008120350086811310599752454820913359125946312765341381171153152850875170563953616312588926210782896304893737980820998854277494438718347902663339151120120657027046197432328902168683080077881837105545672276947861515636560467074205889746346122442073078625034985150166912324856625268061689979087512243374265122200504262834119529487103451165541326299486288085114848709493677473563925554053937376804016377427471623240189434460287558957667530232944863319739907";
        // While a `b` of 834893 works, using a significantly larger `b` will test the intermediate
        // GCD computations
        let b = 2usize.pow(31);
        let n = Int::from_str(n).unwrap();
        assert_eq!(factor(n, b), Ok(Int::from(348242219231u64)));
    }
}
