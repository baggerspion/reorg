import Col from 'react-bootstrap/Col';
import Layout from '../../layouts/Layout';
import Nav from 'react-bootstrap/Nav';
import Row from 'react-bootstrap/Row';
import submissionDetails from '../../gql/submissionDetails';
import Tab from 'react-bootstrap/Tab';
import TabContainer from 'react-bootstrap/TabContainer';
import Table from 'react-bootstrap/Table';
import { useQuery } from '@apollo/react-hooks';
import { withApollo } from '../../libs/apollo';

const Submission = (id) => {
    const submission = id["url"]["query"]["id"]
    const { loading, error, data } = useQuery(submissionDetails(submission));

    if (error) return <h1>Error</h1>;
    if (loading) return <h1>Loading...</h1>;

    return (
        <Layout>
            <h1>{data.findSubmissionByID.title}</h1>
            <hr />
            <TabContainer id="left-tabs-example" defaultActiveKey="first">
                <Row>
                    <Col sm={2}>
                        <Nav variant="pills" className="flex-column">
                            <Nav.Item>
                                <Nav.Link eventKey="first">Content</Nav.Link>
                            </Nav.Item>
                            <Nav.Item>
                                <Nav.Link eventKey="second">Reviews</Nav.Link>
                            </Nav.Item>
                        </Nav>
                    </Col>
                    <Col sm={10}>
                    <Tab.Content>
                        <Tab.Pane eventKey="first">
                            <h2>Authors</h2>
                            <ul>
                                {data.findSubmissionByID.authors.data.map((author) =>
                                    <li>{author.first_name} {author.last_name}</li>
                                )}
                            </ul>
                            <h2>Abstract</h2>
                            <p>{data.findSubmissionByID.abstract}</p>
                        </Tab.Pane>
                        <Tab.Pane eventKey="second">
                            <Table responsive striped bordered hover size="sm">
                                <thead>
                                    <tr>
                                        <th>Reviewer</th>
                                        <th>Review</th>
                                        <th>Notes To Committee</th>
                                        <th style={{textAlign: "center"}}>Score</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.findSubmissionByID.reviews.data.map((review) => (
                                        <tr>
                                            <td>{review.reviewer.reviewer.first_name} {review.reviewer.reviewer.last_name}</td>
                                            <td>{review.public}</td>
                                            <td>{review.private}</td>
                                            <td style={{textAlign: "center"}}>{review.score}</td>
                                        </tr>
                                    ))}
                                </tbody>
                            </Table>
                        </Tab.Pane>
                    </Tab.Content>
                    </Col>
                </Row>
            </TabContainer>
        </Layout>
    );
}

export default withApollo({ ssr: true })(Submission);;
