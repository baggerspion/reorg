import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

const chunk = (arr, size) => (
    Array.from({ length: Math.ceil(arr.length / size) }, (_, i) =>
        arr.slice(i * size, i * size + size)
    )
);

const Reviewers = (props) => {
    return (
        <Container>
            {chunk(props.committee, 4).map((row) => (
                <Row className="mb-4">
                    {row.map((reviewer) => (
                        <div className="col-3">
                            <Card className="h-100">
                                <Card.Img variant="top" src="data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiA/Pjxzdmcgdmlld0JveD0iMCAwIDMyIDMyIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxkZWZzPjxzdHlsZT4uY2xzLTF7ZmlsbDojNjA2MTYxO308L3N0eWxlPjwvZGVmcz48dGl0bGUvPjxnIGRhdGEtbmFtZT0iTGF5ZXIgNyIgaWQ9IkxheWVyXzciPjxwYXRoIGNsYXNzPSJjbHMtMSIgZD0iTTE5Ljc1LDE1LjY3YTYsNiwwLDEsMC03LjUxLDBBMTEsMTEsMCwwLDAsNSwyNnYxSDI3VjI2QTExLDExLDAsMCwwLDE5Ljc1LDE1LjY3Wk0xMiwxMWE0LDQsMCwxLDEsNCw0QTQsNCwwLDAsMSwxMiwxMVpNNy4wNiwyNWE5LDksMCwwLDEsMTcuODksMFoiLz48L2c+PC9zdmc+" />
                                <Card.Body>
                                    <Card.Title>
                                        <span>{reviewer.reviewer.first_name}</span>
                                    </Card.Title>
                                    <Card.Subtitle>{reviewer.reviewer.last_name}</Card.Subtitle>
                                </Card.Body>
                                <Card.Footer>
                                    <span>Still to review:</span><span className="float-right">3</span>
                                </Card.Footer>
                            </Card>
                        </div>
                    ))}
                </Row>
            ))}
        </Container>
    );
}

export default Reviewers;