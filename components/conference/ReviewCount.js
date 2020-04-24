import confReviews from '../../gql/confReviews';
import { useQuery } from '@apollo/react-hooks';
import { withApollo } from '../../libs/apollo';

const ReviewCount = (props) => {
    const { loading, error, data } = useQuery(confReviews(props.conf));

    if (loading) return <span>Loading...</span>;
    if (error) return <span>Error...</span>;

    const result = data.findConferenceByID.submissions.data.filter(result => result['status'] != "SUBMITTED");
    
    return (
        <span>{result.length}</span>
    );
}

export default withApollo({ ssr: true })(ReviewCount);