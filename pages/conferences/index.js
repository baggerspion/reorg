import allConferences from '../../gql/allConferences';
import ConferenceList from '../../components/conference/ConferenceList';
import Layout from '../../layouts/Layout';
import { useQuery } from '@apollo/react-hooks';
import { withApollo } from '../../libs/apollo';

const Conferences = () => {
    const { loading, error, data } = useQuery(allConferences());
    if (error) return <h1>Error</h1>;
    if (loading) return <h1>Loading...</h1>;

    return (
        <Layout>
            <h1>Conferences</h1>
            <ConferenceList data={data} />
        </Layout>
    );
}

export default withApollo({ ssr: true })(Conferences);