pub mod gates;

#[cfg(test)]
mod test {
    use crate::gates::*;

    #[test]
    fn test_nand() {
        assert_eq!(truth_table(NAND), [[1, 1], [1, 0]])
    }

    #[test]
    fn test_not() {
        assert_eq!(NOT(0), 1);
        assert_eq!(NOT(1), 0);
    }

    #[test]
    fn test_and() {
        assert_eq!(truth_table(AND), [[0, 0], [0, 1]])
    }

    #[test]
    fn test_or() {
        assert_eq!(truth_table(OR), [[0, 1], [1, 1]]);
    }

    #[test]
    fn test_xor() {
        assert_eq!(truth_table(XOR), [[0, 1], [1, 0]]);
    }

    #[test]
    fn test_mux() {
        assert_eq!(multi_truth_table(MUX), [0, 0, 1, 1, 0, 1, 0, 1]);
    }

    #[test]
    fn test_demux() {
        assert_eq!(DEMUX(1, 0), (1, 0));
        assert_eq!(DEMUX(1, 1), (0, 1));
        assert_eq!(DEMUX(0, 0), (0, 0));
        assert_eq!(DEMUX(0, 1), (0, 0));
    }

    #[test]
    fn test_multi_not() {
        assert_eq!(
            multi_NOT(vec![0, 0, 0, 0, 1, 1, 1, 1]),
            vec![1, 1, 1, 1, 0, 0, 0, 0]
        )
    }

    #[test]
    fn test_multi_and() {
        assert_eq!(
            multi_AND(&vec![0, 0, 1, 1], &vec![0, 1, 0, 1]),
            vec![0, 0, 0, 1]
        )
    }
}
