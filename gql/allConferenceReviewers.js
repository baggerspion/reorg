import gql from 'graphql-tag';

const allConferenceReviewers = (short_name) => {
    return (
        gql`query allConfReviewers {
            findConferenceByID(id: "${short_name}") {
                name
                reviewers {
                    data {
                        reviewer {
                            first_name
                            last_name
                            email
                        }
                    }
                }
            }   
        }`
    );
}

export default allConferenceReviewers;