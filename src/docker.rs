use std::boxed::Box;
use url::Url;
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

#[derive(Debug)]
pub struct DockerContainerInfo {
    pub id: String,
    pub status: String,
    pub from: String
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

    // TODO improve errors handling
    pub fn new(endpoint_url: &str, labels: &str) -> Result<DockerEvents, &'static str> {
        let url = Url::parse(endpoint_url).unwrap();
        let docker = Docker::host(url);
        let events_options = build_events_options(labels);
        let container_list_options = build_container_list_options(labels);
        Ok(DockerEvents {
            docker,
            events_options,
            container_list_options
        })
    }

    pub fn watch(&self) -> Box<Iterator<Item=DockerContainerInfo> > {
        let iter = self.docker
            .events(&self.events_options)
            .unwrap()
            .map(|evt| DockerContainerInfo {
                id     : evt.id.unwrap(),
                status : evt.status.unwrap(),
                from   : evt.from.unwrap(),
            });

        Box::new(iter)
    }

    pub fn get_containers_info(&self) -> Vec<ContainerDetails>  {

        let containers = self.docker.containers();
        let container_list = containers.list(&self.container_list_options).unwrap();

        let mut infos = Vec::new();

        for container in container_list {
            let details = containers.get(&container.Id).inspect().unwrap();
            infos.push(details);
        }
        infos
    }

    pub fn restart_container(&self, id: &str) {
        let containers = self.docker.containers();
        let container = containers.get(id);
        container.restart(None).unwrap();
    }

}
