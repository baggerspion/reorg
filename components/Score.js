const standardDeviation = (arr, usePopulation = false) => {
    const mean = arr.reduce((acc, val) => acc + val, 0) / arr.length;
    return Math.sqrt(
        arr.reduce((acc, val) => acc.concat((val - mean) ** 2), []).reduce((acc, val) => acc + val, 0) /
        (arr.length - (usePopulation ? 0 : 1))
    ).toFixed(1);
};

const Score = props => {
    let score = 0
    props.data.map((review) =>
        score += review.score
    )
    return (
        <span>{score} <small>({props.data.length} votes, σ={standardDeviation(props.data.map((review) => review.score))})</small></span>
    );
};

export default Score;