import allConferenceReviewers from '../../gql/allConferenceReviewers';
import allConferenceSubmissionScores from '../../gql/allConferenceSubmissionScores';
import Layout from '../../layouts/Layout';
import Reviewers from '../../components/reviewers/Reviewers';
import Submissions from '../../components/submissions/Submissions';
import { useQuery } from '@apollo/react-hooks';
import { withApollo } from '../../libs/apollo';

const Conference = (id) => {
    const conference = id["url"]["query"]["id"]
    const { loading, error, data } = useQuery(allConferenceReviewers(conference));
    const { loading: loadScore, error: errorScore, data: dataScore } = useQuery(allConferenceSubmissionScores(conference));

    if (error || errorScore) return <h1>Error</h1>;
    if (loading || loadScore) return <h1>Loading...</h1>;

    return (
        <Layout>
            <h1>{data.conferenceByShortName.name}</h1>
            <hr />
            <h2>Submissions</h2>
            <Submissions submissions={dataScore.conferenceByShortName.submissions.data} />
            <hr />
            <h2>Reviewers</h2>
            <Reviewers committee={data.conferenceByShortName.reviewers.data} />
        </Layout>
    );
}

export default withApollo({ ssr: true })(Conference);