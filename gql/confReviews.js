import gql from 'graphql-tag';

const confReviews = (conf) => {
    return (
        gql`query confReviews { findConferenceByID(id: ${conf}) { submissions { data { status } } } }`
    );
}

export default confReviews;