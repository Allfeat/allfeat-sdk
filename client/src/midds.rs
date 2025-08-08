use crate::{metadata::melodie, AllfeatOnlineClient};
use allfeat_midds::musical_work::MusicalWork;
use async_trait::async_trait;

#[async_trait]
pub trait AllfeatMiddsRegister {
    type Error;

    async fn register_musical_work<
        Signer: subxt::tx::Signer<subxt::SubstrateConfig> + Send + Sync,
    >(
        &self,
        musical_work: MusicalWork,
        signer: &Signer,
    ) -> Result<subxt::blocks::ExtrinsicEvents<subxt::SubstrateConfig>, Self::Error>;
}

#[async_trait]
impl AllfeatMiddsRegister for AllfeatOnlineClient {
    type Error = subxt::Error;

    async fn register_musical_work<
        Signer: subxt::tx::Signer<subxt::SubstrateConfig> + Send + Sync,
    >(
        &self,
        musical_work: MusicalWork,
        signer: &Signer,
    ) -> Result<subxt::blocks::ExtrinsicEvents<subxt::SubstrateConfig>, Self::Error> {
        // Create the dynamic transaction
        let tx = melodie::tx().musical_works().register(musical_work.into());

        // Submit the transaction and wait for it to be included in a block
        let tx_progress = self
            .tx()
            .sign_and_submit_then_watch_default(&tx, signer)
            .await?;

        // Wait for the transaction to be finalized and return the events
        let tx_events = tx_progress.wait_for_finalized_success().await?;

        Ok(tx_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use allfeat_midds::musical_work::types::*;
    use std::str::FromStr;
    use subxt::{OnlineClient, SubstrateConfig};
    use subxt_signer::sr25519::dev;

    #[tokio::test]
    async fn test_register_musical_work_local_node() -> Result<(), Box<dyn std::error::Error>> {
        // Connexion au nœud local (port par défaut)
        let client = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

        // Utilisation du compte de développement Alice
        let signer = dev::alice();

        // Création d'une œuvre musicale de test
        let musical_work = MusicalWork {
            iswc: Iswc::from_str("T-1234567890")?,
            title: MusicalWorkTitle::from_str("Test Musical Work")?,
            creation_year: Some(2024),
            instrumental: Some(false),
            language: Some(allfeat_midds::shared::Language::French),
            bpm: Some(120),
            key: Some(allfeat_midds::shared::Key::C),
            work_type: Some(MusicalWorkType::Original),
            participants: {
                let mut participants = MusicalWorkParticipants::new();
                participants.push(Participant {
                    id: 1,
                    role: ParticipantRole::Composer,
                })?;
                participants.push(Participant {
                    id: 2,
                    role: ParticipantRole::Author,
                })?;
                participants
            },
            classical_info: None,
        };

        // Validation de l'œuvre musicale
        use allfeat_midds::shared::conversion::Validatable;
        musical_work.validate()?;

        // Enregistrement de l'œuvre musicale sur la blockchain
        let tx_events = client.register_musical_work(musical_work, &signer).await?;

        // Vérification que la transaction a été soumise avec succès
        println!("Transaction submitted successfully!");
        // println!("Block hash: {:?}", tx_events.block_hash());

        // Recherche d'événements spécifiques
        for event in tx_events.iter() {
            let event = event?;
            println!("Event: {:?}", event);

            // Vérification de l'événement de succès de l'enregistrement
            if event.pallet_name() == "MusicalWorks"
                && event.variant_name() == "MusicalWorkRegistered"
            {
                println!("✅ Musical work registered successfully!");
            }
        }

        Ok(())
    }

    // Helper function pour attendre que le nœud soit prêt
    async fn wait_for_node_ready() -> Result<(), Box<dyn std::error::Error>> {
        let max_retries = 10;
        let mut retries = 0;

        loop {
            match OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await {
                Ok(_) => {
                    println!("✅ Nœud local connecté avec succès");
                    return Ok(());
                }
                Err(e) => {
                    retries += 1;
                    if retries >= max_retries {
                        return Err(format!(
                            "Impossible de se connecter au nœud après {} tentatives: {}",
                            max_retries, e
                        )
                        .into());
                    }
                    println!(
                        "Tentative de connexion {}/{} échouée, nouvelle tentative dans 2s...",
                        retries, max_retries
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_node_connection() -> Result<(), Box<dyn std::error::Error>> {
        wait_for_node_ready().await?;
        Ok(())
    }
}
