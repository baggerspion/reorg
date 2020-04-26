import allConferenceReviewers from '../../gql/allConferenceReviewers';
import Layout from '../../layouts/Layout';
import Reviewers from '../../components/reviewers/Reviewers';
import { useQuery } from '@apollo/react-hooks';
import { withApollo } from '../../libs/apollo';

const Conference = (id) => {
    const conference = id["url"]["query"]["id"]
    const { loading, error, data } = useQuery(allConferenceReviewers(conference));
    if (error) return <h1>Error</h1>;
    if (loading) return <h1>Loading...</h1>;

    return (
        <Layout>
            <h1>{data.conferenceByShortName.name}</h1>
            <hr />
            <h2>Reviewers</h2>
            <Reviewers committee={data.conferenceByShortName.reviewers.data} />
        </Layout>
    );
}

export default withApollo({ ssr: true })(Conference);