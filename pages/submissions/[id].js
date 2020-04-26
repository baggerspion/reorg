import Layout from '../../layouts/Layout';
import submissionDetails from '../../gql/submissionDetails';
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
        </Layout>
    );
}

export default withApollo({ ssr: true })(Submission);;
