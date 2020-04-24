import gql from 'graphql-tag';

const allConferences = () => {
    return (
        gql`query allConferences { allConferences { data { _id, name, short_name } } }`
    );
}

export default allConferences;