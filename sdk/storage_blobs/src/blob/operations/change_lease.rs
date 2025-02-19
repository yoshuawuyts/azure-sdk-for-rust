use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

operation! {
    ChangeLease,
    client: BlobLeaseClient,
    proposed_lease_id: ProposedLeaseId,
}

impl ChangeLeaseBuilder {
    pub fn into_future(mut self) -> ChangeLease {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "lease");

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "change");
            headers.add(self.client.lease_id());
            headers.add(self.proposed_lease_id);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            ChangeLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(ChangeLeaseResponse ,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    lease_id_from_headers => lease_id: LeaseId,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);
