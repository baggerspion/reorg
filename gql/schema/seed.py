from dotenv import load_dotenv

from itertools import zip_longest

from faker import Faker

from faunadb import query as q
from faunadb.objects import Ref
from faunadb.client import FaunaClient

import os, random, time

load_dotenv()
client = FaunaClient(os.environ['FAUNADB_SECRET'])
fake = Faker(['it_IT', 'en_GB', 'de_DE', 'cs_CZ'])

def create_authors():
    authors = []
    users = client.query(q.paginate(q.match(q.index("allUsers")), size=1000))
    submissions = client.query(q.paginate(q.match(q.index("allSubmissions")), size=1000))

    for paper in submissions['data']:
        for author in random.sample(users['data'], random.randint(0, 1) + 1):
            new_author = {
                "submissionID": paper,
                "userID": author
            }
            authors.append(new_author)

    return authors

def create_conferences():
    conferences = []

    for _ in range(20):
        new_conference = {
            "name": fake.bs(),
            "short_name": fake.slug(),
            "start_date": q.date(fake.date()),
            "end_date": q.date(fake.date()),
            "venue": fake.company(),
            "address": fake.address(),
            "country": fake.country(),
            "email": fake.ascii_free_email(),
            "web": fake.uri(),
        }
        conferences.append(new_conference)

    return conferences

def create_reviewers():
    reviewers = []
    users = client.query(q.paginate(q.match(q.index("allUsers")), size=1000))
    conferences = client.query(q.paginate(q.match(q.index("allConferences")), size=1000))

    for conf in conferences['data']:
        for user in random.sample(users['data'], 7):
            new_committee_member = {
                "conference": conf,
                "reviewer": user
            }
            reviewers.append(new_committee_member)

    return reviewers

def create_reviews():
    reviews = []
    conferences = client.query(q.paginate(q.match(q.index("allConferences")), size=1000))

    for conf in conferences['data']:
        reviewers = client.query(q.paginate(q.match(q.index("conference_reviewers_by_conference"), conf), size=1000))
        for reviewer in reviewers['data']:
            submissions = client.query(q.paginate(q.match(q.index("conference_submissions_by_conference"), conf), size=1000))
            for submission in submissions['data']:
                new_review = {
                    "reviewer": reviewer,
                    "submission": submission,
                    "score": random.randint(0, 6) + 1,
                    "public": fake.paragraph(),
                    "private": fake.paragraph()
                }
                reviews.append(new_review)

    return reviews

def create_submissions():
    submissions = []
    conferences = client.query(q.paginate(q.match(q.index("allConferences")), size=1000))

    for conference in conferences['data']:
        for _ in range(random.randint(1, 30)):
            new_submission = {
                "conference": conference,
                "accepted": False,
                "title": fake.sentence(nb_words=random.randint(3, 6) + 1),
                "abstract": fake.paragraph(),
                "status": random.sample(["SUBMITTED", "SELECTED", "REJECTED", "WAITING"] , 1)
            }
            submissions.append(new_submission)

    return submissions

def create_users():
    users = []

    for _ in range(1000):
        new_user = {
            "first_name": fake.first_name(),
            "last_name": fake.last_name(),
            "email": fake.ascii_free_email()
        }
        users.append(new_user)

    return users

def insert_data(collection, data, batchSize):
    def batcher(iterable, n, fillvalue = None):
        args = [iter(iterable)] * n
        return zip_longest(*args, fillvalue =fillvalue)

    count = 1
    for batch in batcher(data, batchSize):
        clean_batch = [i for i in batch if i]
        print("Inserting [%s] %s to %s" % (collection, count, count + len(clean_batch)))
        count = count + len(clean_batch)

        client.query(
            q.map_(
                lambda item: q.create(
                    q.collection(collection),
                    {"data": item}
                ),
                clean_batch
            )
        )

if __name__ == '__main__':
    insert_data("User", create_users(), 100)
    insert_data("Conference", create_conferences(), 100)
    insert_data("Reviewer", create_reviewers(), 100)
    insert_data("Submission", create_submissions(), 100)
    insert_data("Author", create_authors(), 100)
    insert_data("Review", create_reviews(), 20)