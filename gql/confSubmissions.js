import gql from 'graphql-tag';

const confSubmissions = (conf) => {
    return (
        gql`query confSubmissions { findConferenceByID(id: ${conf}) { submissions { data { _id } } } }`
    );
}

export default confSubmissions;