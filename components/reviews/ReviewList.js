import Table from 'react-bootstrap/Table'

const ReviewList = (props) => {
    return (
        <Table responsive striped bordered hover size="sm">
            <thead>
                <tr>
                    <th>Reviewer</th>
                    <th style={{textAlign: "center"}}>Score</th>
                </tr>
            </thead>
            <tbody>
                {props.reviews.map((review) => (
                    <>
                    <tr>
                        <th>{review.reviewer.reviewer.first_name} {review.reviewer.reviewer.last_name}</th>
                        <th style={{textAlign: "center"}}>{review.score}</th>
                    </tr>
                    <tr>
                        <td colSpan="2">{review.public}</td>
                        <td colSpan="2">{review.private}</td>
                    </tr>
                    </>
                ))}
            </tbody>
        </Table>
    );
}

export default ReviewList;