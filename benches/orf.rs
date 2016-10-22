#![feature(test)]

extern crate test;
extern crate bio;

use test::Bencher;

use bio::seq_analysis::orf::Finder;
use bio::seq_analysis::gc::*;


// 5,000 random nucleotides, GC content = .55
static STR_1: &'static [u8] = b"ATCTAACTATTCCCTGTGCCTTATGGGGGCCTGCGCTATCTGCCTGT\
CGAACCATAGGACTCGCGCCAGCGCGCAGGCTTGGATCGAGGTGAAATCTCCGGGGCCTAAGACCACGAGCGTCTGGCG\
TCTTGGCTAACCCCCCTACATGCTGTTATAGACAATCAGTGGAAACCCGGTGCCAGGGGGTGGAGTGACCTTAAGTCAG\
GGACGATATTAATCGGAAGGAGTATTCAACGCAATGAAGCCGCAGGGTTGGCGTGGGAATGGTGCTTCTGTCCAAGCAG\
GTAAGGGCATGAGGCCGCAACCGTCCCCCAAGCGTACAGGGTGCACTTTGCAACGATTTCGGAGTCCGGAGACTCGCTG\
TTTTCGAAATTTGCGCTCAAGGGCGGGTATTGAACCAGGCTTACGCCCAAGAACGTAGCAAGGTGACTCAAACAAGGTA\
CATCTTGCCCGCGTTTCACACGAATCAAGTTGGAGGTTATGGAGCATAGTAACACGTGGGCGGCCAGTGGTCGGTTGCT\
ACACCCCTGCCGCAACGTTGAAGGTCCCGGATTAGACTGGCTGGACCCATGCCGTGACACCCGTCACACTCCATTACCG\
TCTGCGGGTCACGGCTTGTTGTGGACTGGATTGCCATTCTCTCAGTGTATTACGCAGGCCGGCGCGCGGGTCCCATGTA\
AACCTGTCATAGCTTACCTGACTCTACTTGGAAGTGTGGCTAGGCCTTTGCCCACGCACCTGGTCGGTCCTCGTTTGCT\
TTTTAGGACCGGATGAACTACAGAGCGCTGCAAGAATCTCTACCTGCTTTACAAAGCGCTGGGTCCTACTCCAGCGGGA\
TGTTTTATCTAAACACGATGAGAGGAGTATTCGTCAGGCCACATGGCTTTCTTGTCCTGGTCGGATCCATCGTTGGCGC\
CCGACCCCCCCACTCCGTAGTGAGTTCTTCGTCCGAGCCATTGCATGCCAGATCGGCAGACAGATAGCGGATCCAGTAT\
ATCCCTGGAAGCTATAGACGCACAGGTTGGAATCCTAAGCGAAGTCGCGCGTCCGAACCCAGCTCTACTTTAGTGGCCA\
CGGGTTCTGGTCCCCCCGGGCCGCGGAACCGATTAGGGCCATGTACAACAATACTTATTAGTCACCTTTCAGACACGAT\
CTCCCTGCTCAGTGGTATATGGTTCCTGCTATAATTAGCCACCCTCATAAGTTGCACTACTTCTGCGACCCAAGTGCAC\
CCTTACCACGAAGACAGGATTGTCCGATCCCATACTGCGGCCTTGGCAGGGGGTTCGCAAGTCCCACCCCAAACGATGC\
TGAAGGCTCAGGTTACACAGGCACAAGTGCTATATACGCGAGTTCCCGCTCTTAACCTGGACCGAATGCGGGATCATGC\
ATCGTACCACTGTGTTCGTGTCATCTAGGACGGGCGCAAAGGATACATAGTTCAATCAAGAATACCTTGTATTATTGTA\
CACCTACCGGTCACCAGCCAACAATGTGCGGACGGCGTTGCGACTTGCTGGGCCTGATCTCACCGCCCTAGATACCGCA\
CACTGGGCAATACGAGGTAAAGCCAGTCACCCAGTGTCGATCAACAGCTGACGTAACGGTAAGAGGCTCACAAAATCGC\
ACCGCCGGCGTCCCCTGGGTATTTTACGTCAGCATCGGGTGGACTGGCATGAATCTTTACTCCCAGGCGGAAACGGGTG\
CGTGGACAAGCGAGCAGCAAACGAAAATTCCTGGCCTGCTTGGTGTCTCGTATCCCTCTTGGAGATCGAGGAAATGTTT\
CACGACCAAGGGAAAGGTCGCCCTACGAAATAGATTTGCGCTACTGTCCGCATAAGGAGTCCGGTGTAGCGAAGGATGA\
AGGCGACCCTAGGTAGCAACCGCCGGCTTCGGCGGTAAGGTATCACTCAGGAAGCAGGCACGGAAAGACACGGTCTAGC\
AGACCGTCTATCGGCTAGGTCAAATAGGGTGCTTTGATATCAGCATGTCCAGCCTTAGAATTCAGTTCAGCGCGCTGGT\
CTGGGTCGAGATAAAATCACCAGTACCCAAGACCAGGCGGGCTCGCCGCGTTGGCTAATCCTGGTACATCTTGTAATCA\
ATGTTCAGAAGAAAATCTGTGTTAGAGGGACGAGTCACCACGTACCAATAGCGACAACGATCGGTCGGACTATTCATCG\
TGGTGGTGACGCTCGGATTACGCGGGAAAGGTGCTTGTGTCCCGACAGGCTAGGATATAATGCTGAGGCGCTGCCCCAA\
CCGTTCAGCGTGGGGTTTGCTACAACTTCCGAGTGCTACGTGTGCGAGACCATGTTATGTATGCACAAGGCCGACAATA\
GGACGTAGCCTTCGAGTTAGTACGTAGCGTGGTCGCACAAGCACAGTAGATCCTCCCCGCGCATCCTATTTATTAAGTT\
AATTCTATAGCAATACGATCACATGCGGATGGGCAGTGGCCGGTAGTCACACGCCTACCGCGGTGCTCAATGACCGGGA\
CTAGAGAGGCGAAGATTATGGCGTGTGACCCGTTATGCTCGAGTTCGGTCAGAGCGTCATTGCGAGTAGTCGATTGCTT\
TCCCAATCTCCGAGCGATTTAGCGTGACAGCCCCAGGGAACCCACAAAATGCGATCGCAGTCCACCCGATCGTACACAG\
AAAGGAGGGTCCCCATACGCCGACGCACCTGTTCGCACGTCGTATGCATAAACGAGCCGCACGAACCAGAGAGCATAAA\
GAGGACCTCTAGCTCCTTTACAAAGTACAGGTTCGCCTGCCGCCGGGATGCCTTACCTAGACGCAATGACGGACGTATT\
CCTCTGGCCTCAACGGTTCCTGCTTCCGCTGGGATCCAAGATTGGCGGCCGAAGCCGCCTTTCCAAAGTGAGTCCTTCG\
TCTGTGACTAACTGTGCCAGATCGTCTTGCAAACTCCCGATCCAGTTTAACTCACCAAACTATAGCCGTACAGACCCAA\
ATCTTAAGTCATATCACGCGACTAGCCTCTGCTCAATTTCTGTGCTCAAGGGTTTTGGTCCGCCCGAGCGGTGCAGCCG\
ATTAGGACCATCTAATGCACTTGTTACAAGACTTCTTTTAAACACTTTCTTCCTGCCCAGTGGCGGATGATAATGGTTG\
TTGCCAGCCGGCGTGGAAGGTAACAGCACCGGTGCGAGCCTAATGTGCCGTCTCCACCAACACAGGGCTGTCCGGTCGT\
ATAATAGGACTCCGCAATGGGGTTAGCAAGTGGCAGCCTAAACGATGTCGGGGACTCGCGATGTACATGCTCTGGTTCA\
ATACATACGTGACCCGGCAGTTATCCTGCATCGGAACGTCAATCGTGCATCGGGCCAGCGTAATCGTGTCATCTGGGAG\
GCGGCCGTAGGATAAATAATTCAATAAAGATGTCGTTTTGCTAGTATACGCCTAGGCGTCACCCGCCATCTCTGTGCAG\
GTGGGCCGACGAGACACTGCCCCTGATTTCTCCGCTACTAATAGCACACACGGGGCAATACCAGCACAAGCCAGTCTCG\
CGGGAACGCTCGTCAGCATACGAAAGAGCTTGAGGCACGCCAATTCGCACTGTCGGGGTCGCTTGGGTGTTTTGCACTA\
CCGTCAGGTACGCTAGTATGCGTCCTTCCTTCCAGGGGTATGTGGCTGCGTGGTCAAAAGTGCGGCATTCGTATTTGCT\
CCCCGTGCTTGCTCTCACGAACTTGACCTGGAGATCAAGGAGATGCTTCTTGTGGAACCGGACAGCGCATCAACGCAAC\
GGATCTACGTTACAGCGTGCATAGCGAGAACGGAGTTGCCGACGACGAAAGCGACACTGGGATCTGTCCGTCGTCATTC\
GCGGAAAGCATCCGCTCACGAGGCGGACACTGATTGACACGGTTTTGCAGAAGGTTAGGGGAATAGGTCAAATTGAGTG\
GCTTAAAAACGCTATGTCTGGGATTAAAGTGTAGTAAACTGCGGTCAACGGAGACGGTTTTAAGACAGGAGTTCGCAAA\
ACCAGGCGGGGTCGCCACGACGGCTATTCCTGGTGGTTTAGGCGTACAATGTCCTGAAGAATATTTAAGAAAGAAGCAC\
CCCTCGTCGCCTAGAATTACCTACCGCGGTCGACCATACCTTCGATTGTCGCGCCCACCCTCCCATTAGTCGGCAGAGG\
TGGTTGTGTTGCGATAGCCCAGCATGATATCCTAAGGCGTTACGCCGATGGATATCCCACGGAATTGCCATAGGCGCTG\
AACGCTACACGGACGATACGAACTTATGTATGGAGCGGGTCATCGAAAGGTCATACCCTTGTAGTTAACATGTAGCCCG\
GCCCTATTAGTACAGCAGTGCCTTGAGCGGCATTCTCATTATTAAGTTTTCTCTACAGCCAAACGACCAAGTGCACTTC\
CGCGGAGCGCGGTGGAGACTCGTCCACCCGGCAGCTCTGTAATAGGGACTAAAAGAGTGATGATAATCATGAGTGCCGC\
GTTATGGTGGTGTCGGAACAGAGCGGTCTTACGGCCAGTCGTATCCCTTCTCGAGTTCCGTCCGGTTAAGCGTGACACT\
CCCAGTGTACCCGCAAACCGTGATGGCTGTGCTTGGGGTCAATCGCATGTAGGATGGTCTCCAGACACCGGGGCACCAG\
TTTTCACGCCCAAAGCATAAACGACGAGCAGTCATGAGAGTCTTAGAACTGGACGTGCCGTTTCTCTGCGAACAACACC\
TCGAGCTGTACCGTTGTTGCGCTGCCTAGATGCAGTGCCGCTCCTATCACATTTGCCTCGACGACTGCCGCCTTCGCTG\
TTTCCCTAGACACTCAACAGTAAGCGCCTTTTGTAGGCAGGGGCACCCCCTGTCAGTGGCTGCGCCAAAACGTCTTCGG\
ATCCCCTTGTCCAATCAAACTGACCGAATTCTTTCATTTAAGACCCTAATATGACATCATTAGTGACTAAATGCCACTC\
CCAAAATTCTGCCCAGAAGCGTTTAAGTTCGCCCCACTAAAGTTGTCTAAAACGA";

#[bench]
fn bench_orf(b: &mut Bencher) {
    let start_codons = vec!(b"ATG");
    let stop_codons  = vec!(b"TGA", b"TAG", b"TAA");
    let finder = Finder::new(start_codons, stop_codons, 100usize);
    b.iter(|| { finder.find_all(STR_1).count() });
}

#[bench]
fn bench_gc(b: &mut Bencher) {
	b.iter(|| gc_content(STR_1));
}