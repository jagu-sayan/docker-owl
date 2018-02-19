use failure::Error;

use futures::{prelude::*};
use futures::compat::Compat01As03;

use shiplift::Docker;
use shiplift::builder::ContainerListOptions;
use shiplift::builder::ContainerFilter;
use shiplift::builder::EventsOptions;
use shiplift::builder::EventFilterType;
use shiplift::builder::EventFilter;
use shiplift::rep::ContainerDetails;


pub struct DockerEvents {
    pub docker: Docker,
    pub events_options: EventsOptions,
    pub container_list_options: ContainerListOptions,
}


fn build_events_options(labels: &str) -> EventsOptions {
    let mut options = EventsOptions::builder();
    let mut filters = vec![
        EventFilter::Type(EventFilterType::Container),
        EventFilter::Event("start".to_string()),
        EventFilter::Event("stop".to_string()),
        EventFilter::Event("die".to_string()),
    ];
    for label in labels.split_whitespace() {
        filters.push(EventFilter::Label(label.to_string()));
    }
    options.filter(filters);
    options.build()
}

fn build_container_list_options(labels: &str) -> ContainerListOptions {
    let mut options = ContainerListOptions::builder();
    let mut filters: Vec<ContainerFilter> = vec![
        ContainerFilter::Status("running".to_string()),
    ];
    for label in labels.split_whitespace() {
        filters.push(ContainerFilter::LabelName(label.to_string()));
    }
    options.filter(filters);
    options.build()
}


impl DockerEvents {

    pub fn new(_endpoint_url: &str, labels: &str) -> Result<DockerEvents, Error> {
        // let url = endpoint_url.parse::<Uri>().context(format!("Unable to parse '{}' url", endpoint_url))?;
        // let docker = Docker::host(url);
        let docker = Docker::new();
        let events_options = build_events_options(labels);
        let container_list_options = build_container_list_options(labels);
        Ok(DockerEvents {
            docker,
            events_options,
            container_list_options
        })
    }

    pub fn watch(&self) -> impl Stream<Item = std::result::Result<shiplift::rep::Event, shiplift::Error>> {
        Compat01As03::new(self.docker
            .events(&self.events_options))
    }

    pub async fn get_containers_info(&self) -> Vec<ContainerDetails> {

        let containers = self.docker.containers();
        let container_list = await!(
            Compat01As03::new(containers.list(&self.container_list_options))
        ).unwrap();

        let mut infos = Vec::new();

        for container in container_list {
            let details =  await!(
                Compat01As03::new(containers.get(&container.id).inspect())
            ).unwrap();
            infos.push(details);
        }
        infos
    }

    pub fn restart_container<'a>(&'a self, id: &'a str) -> impl Future {
        let containers = self.docker.containers();
        let container = containers.get(id);
        Compat01As03::new(container.restart(None))
    }

}
