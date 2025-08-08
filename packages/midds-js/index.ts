/**
 * Allfeat SDK WebAssembly Bindings
 * Organized exports for better developer experience
 */

// Import from the generated bindings (bundler target)
import * as WasmBindings from "./dist/allfeat_wasm_bindings.js";

// Re-export everything from the base bindings
export * from "./dist/allfeat_wasm_bindings.js";

// Client namespace - blockchain interaction functionality
export namespace Client {
    export const AllfeatClient = WasmBindings.AllfeatClient;
    export type AllfeatClient = WasmBindings.AllfeatClient;

    // Transaction functionality
    export namespace Transactions {
        export const AllfeatTxSystem = WasmBindings.AllfeatTxSystem;
        export type AllfeatTxSystem = WasmBindings.AllfeatTxSystem;

        export const AllfeatTxMusicalWorks = WasmBindings.AllfeatTxMusicalWorks;
        export type AllfeatTxMusicalWorks = WasmBindings.AllfeatTxMusicalWorks;

        export const AllfeatTxPartyIdentifiers =
            WasmBindings.AllfeatTxPartyIdentifiers;
        export type AllfeatTxPartyIdentifiers =
            WasmBindings.AllfeatTxPartyIdentifiers;

        export const AllfeatTxReleases = WasmBindings.AllfeatTxReleases;
        export type AllfeatTxReleases = WasmBindings.AllfeatTxReleases;

        export const AllfeatTxTracks = WasmBindings.AllfeatTxTracks;
        export type AllfeatTxTracks = WasmBindings.AllfeatTxTracks;

        export const Call = WasmBindings.Call;
        export type Call = WasmBindings.Call;

        export const CallWithSigner = WasmBindings.CallWithSigner;
        export type CallWithSigner = WasmBindings.CallWithSigner;

        export const Signer = WasmBindings.Signer;
        export type Signer = WasmBindings.Signer;

        export const SubmittableTransaction =
            WasmBindings.SubmittableTransaction;
        export type SubmittableTransaction =
            WasmBindings.SubmittableTransaction;

        export const Tx = WasmBindings.Tx;
        export type Tx = WasmBindings.Tx;
    }
}

// MIDDS namespace - Music Industry Data Structures
export namespace Midds {
    // Core MIDDS types
    export namespace Core {
        export const Date = WasmBindings.Date;
        export type Date = WasmBindings.Date;

        // Shared enums
        export const Country = WasmBindings.Country;
        export type Country = WasmBindings.Country;

        export const Key = WasmBindings.Key;
        export type Key = WasmBindings.Key;

        export const Language = WasmBindings.Language;
        export type Language = WasmBindings.Language;

        export const GenreId = WasmBindings.GenreId;
        export type GenreId = WasmBindings.GenreId;
    }

    // Musical Work related types
    export namespace MusicalWork {
        export const MusicalWork = WasmBindings.MusicalWork;
        export type MusicalWork = WasmBindings.MusicalWork;

        export const MusicalWorkTitle = WasmBindings.MusicalWorkTitle;
        export type MusicalWorkTitle = WasmBindings.MusicalWorkTitle;

        export const MusicalWorkParticipants =
            WasmBindings.MusicalWorkParticipants;
        export type MusicalWorkParticipants =
            WasmBindings.MusicalWorkParticipants;

        export const Iswc = WasmBindings.Iswc;
        export type Iswc = WasmBindings.Iswc;

        export const DerivedWorks = WasmBindings.DerivedWorks;
        export type DerivedWorks = WasmBindings.DerivedWorks;

        export const ClassicalInfo = WasmBindings.ClassicalInfo;
        export type ClassicalInfo = WasmBindings.ClassicalInfo;

        export const Opus = WasmBindings.Opus;
        export type Opus = WasmBindings.Opus;

        export const Participant = WasmBindings.Participant;
        export type Participant = WasmBindings.Participant;

        export const ParticipantRole = WasmBindings.ParticipantRole;
        export type ParticipantRole = WasmBindings.ParticipantRole;
    }

    // Party Identifier related types
    export namespace PartyIdentifier {
        export const PartyIdentifier = WasmBindings.PartyIdentifier;
        export type PartyIdentifier = WasmBindings.PartyIdentifier;

        export type PartyType = WasmBindings.PartyType;

        // Artist types
        export const Artist = WasmBindings.Artist;
        export type Artist = WasmBindings.Artist;

        export const ArtistAlias = WasmBindings.ArtistAlias;
        export type ArtistAlias = WasmBindings.ArtistAlias;

        export const ArtistAliases = WasmBindings.ArtistAliases;
        export type ArtistAliases = WasmBindings.ArtistAliases;

        export const ArtistFullName = WasmBindings.ArtistFullName;
        export type ArtistFullName = WasmBindings.ArtistFullName;

        export const ArtistGender = WasmBindings.ArtistGender;
        export type ArtistGender = WasmBindings.ArtistGender;

        export const ArtistType = WasmBindings.ArtistType;
        export type ArtistType = WasmBindings.ArtistType;

        // Entity types
        export const Entity = WasmBindings.Entity;
        export type Entity = WasmBindings.Entity;

        export const EntityName = WasmBindings.EntityName;
        export type EntityName = WasmBindings.EntityName;

        export const EntityType = WasmBindings.EntityType;
        export type EntityType = WasmBindings.EntityType;

        // Identifiers
        export const Isni = WasmBindings.Isni;
        export type Isni = WasmBindings.Isni;
    }

    // Release related types
    export namespace Release {
        export const Release = WasmBindings.Release;
        export type Release = WasmBindings.Release;

        export const Ean = WasmBindings.Ean;
        export type Ean = WasmBindings.Ean;

        export const CatalogNumber = WasmBindings.CatalogNumber;
        export type CatalogNumber = WasmBindings.CatalogNumber;

        export const ReleaseTracks = WasmBindings.ReleaseTracks;
        export type ReleaseTracks = WasmBindings.ReleaseTracks;

        export const ReleaseProducers = WasmBindings.ReleaseProducers;
        export type ReleaseProducers = WasmBindings.ReleaseProducers;

        export const ReleaseDistributor = WasmBindings.ReleaseDistributor;
        export type ReleaseDistributor = WasmBindings.ReleaseDistributor;

        export const ReleaseManufacturer = WasmBindings.ReleaseManufacturer;
        export type ReleaseManufacturer = WasmBindings.ReleaseManufacturer;

        export const ReleaseCoverContributor =
            WasmBindings.ReleaseCoverContributor;
        export type ReleaseCoverContributor =
            WasmBindings.ReleaseCoverContributor;

        export const ReleaseCoverContributors =
            WasmBindings.ReleaseCoverContributors;
        export type ReleaseCoverContributors =
            WasmBindings.ReleaseCoverContributors;

        // Release enums
        export const ReleaseFormat = WasmBindings.ReleaseFormat;
        export type ReleaseFormat = WasmBindings.ReleaseFormat;

        export const ReleasePackaging = WasmBindings.ReleasePackaging;
        export type ReleasePackaging = WasmBindings.ReleasePackaging;

        export const ReleaseStatus = WasmBindings.ReleaseStatus;
        export type ReleaseStatus = WasmBindings.ReleaseStatus;

        export const ReleaseType = WasmBindings.ReleaseType;
        export type ReleaseType = WasmBindings.ReleaseType;
    }

    // Track related types
    export namespace Track {
        export const Track = WasmBindings.Track;
        export type Track = WasmBindings.Track;

        export const Isrc = WasmBindings.Isrc;
        export type Isrc = WasmBindings.Isrc;

        export const TrackTitle = WasmBindings.TrackTitle;
        export type TrackTitle = WasmBindings.TrackTitle;

        export const TrackTitleAliases = WasmBindings.TrackTitleAliases;
        export type TrackTitleAliases = WasmBindings.TrackTitleAliases;

        export const TrackContributors = WasmBindings.TrackContributors;
        export type TrackContributors = WasmBindings.TrackContributors;

        export const TrackGenres = WasmBindings.TrackGenres;
        export type TrackGenres = WasmBindings.TrackGenres;

        export const TrackPerformers = WasmBindings.TrackPerformers;
        export type TrackPerformers = WasmBindings.TrackPerformers;

        export const TrackProducers = WasmBindings.TrackProducers;
        export type TrackProducers = WasmBindings.TrackProducers;

        export const TrackRecordingPlace = WasmBindings.TrackRecordingPlace;
        export type TrackRecordingPlace = WasmBindings.TrackRecordingPlace;

        export const TrackMixingPlace = WasmBindings.TrackMixingPlace;
        export type TrackMixingPlace = WasmBindings.TrackMixingPlace;

        export const TrackMasteringPlace = WasmBindings.TrackMasteringPlace;
        export type TrackMasteringPlace = WasmBindings.TrackMasteringPlace;

        export const TrackVersion = WasmBindings.TrackVersion;
        export type TrackVersion = WasmBindings.TrackVersion;
    }
}

// Utility functions
export namespace Utils {
    export const getSdkVersion = WasmBindings.getSdkVersion;
}

// Convenience exports
export const AllfeatClient = WasmBindings.AllfeatClient;

// Default export
export default {
    Client,
    Midds,
    Utils,
    AllfeatClient,
};
