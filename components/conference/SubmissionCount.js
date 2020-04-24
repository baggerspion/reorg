import confSubmissions from '../../gql/confSubmissions';
import { useQuery } from '@apollo/react-hooks';
import { withApollo } from '../../libs/apollo';

const SubmissionCount = (props) => {
    const { loading, error, data } = useQuery(confSubmissions(props.conf));

    if (loading) return <span>Loading...</span>;
    if (error) return <span>Error...</span>;
    
    return (
        <span>{data.findConferenceByID.submissions.data.length}</span>
    );
}

export default withApollo({ ssr: true })(SubmissionCount);