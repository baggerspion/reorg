import Table from 'react-bootstrap/Table'

const mean = (arr) => {
    return Number.parseFloat(arr.reduce((a, b) => a + b, 0) / arr.length).toPrecision(2);
}

const standardDeviation = (values) => {
    var avg = mean(values);
    
    var squareDiffs = values.map(function(value){
        var diff = value - avg;
        var sqrDiff = diff * diff;
        return sqrDiff;
    });
    
    var avgSquareDiff = mean(squareDiffs);
  
    var stdDev = Math.sqrt(avgSquareDiff);
    return Number.parseFloat(stdDev).toPrecision(2);
}

const Submissions = (props) => {
    return (
        <Table responsive striped bordered hover size="sm">
            <thead>
                <tr>
                    <th>Title</th>
                    <th style={{textAlign: "center"}}>#</th>
                    <th style={{textAlign: "center"}}>μ</th>
                    <th style={{textAlign: "center"}}>σ</th>
                </tr>
            </thead>
            <tbody>
                {props.submissions.map((submission) => (
                    <tr>
                        <td>{submission.title}</td>
                        <td style={{textAlign: "center"}}>{submission.reviews.data.length}</td>
                        <td style={{textAlign: "center"}}>{mean(submission.reviews.data.map(score => score['score']))}</td>
                        <td style={{textAlign: "center"}}>{standardDeviation(submission.reviews.data.map(score => score['score']))}</td>
                    </tr>
                ))}
            </tbody>
        </Table>
    )
}

export default Submissions;