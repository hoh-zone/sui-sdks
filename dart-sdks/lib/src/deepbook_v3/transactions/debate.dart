import '../sui/transactions.dart';
import 'config.dart';

/// Debates 功能 for Dart SDK
class DebateContract {
  DebateContract(this.config);

  final DeepBookConfig config;

  /// Vote on a debate proposition
  Map<String, dynamic> vote(Transaction tx, String debateId, bool vote) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::debate::vote',
      [
        tx.pure(encodeU128(debateId)),
        tx.pure([vote ? 1 : 0]),
      ],
    );
  }

  /// Create a new debate
  Map<String, dynamic> create(
      Transaction tx, String topic, String description) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::debate::create',
      [
        tx.pureUtf8String(topic),
        tx.pureUtf8String(description),
      ],
    );
  }

  /// Execute a passed debate
  Map<String, dynamic> execute(Transaction tx, String debateId) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::debate::execute',
      [
        tx.pure(encodeU128(debateId)),
      ],
    );
  }

  /// End a debate and process the outcome
  Map<String, dynamic> end(Transaction tx, String debateId) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::debate::end',
      [
        tx.pure(encodeU128(debateId)),
      ],
    );
  }

  /// Get debate information
  Map<String, dynamic> getDebate(
      Transaction tx, String debateId, String registryId) {
    final poolObject = tx.object(registryId);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::debate::get_debate',
      [
        tx.pure(encodeU128(debateId)),
        poolObject,
      ],
      [
        '${config.packageIds.deepbookPackageId}::debate::Debate',
      ],
    );
  }

  /// Get votes for a debate
  Map<String, dynamic> getVotes(
      Transaction tx, String debateId, String registryId) {
    final poolObject = tx.object(registryId);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::debate::get_votes',
      [
        tx.pure(encodeU128(debateId)),
        poolObject,
      ],
      [
        '${config.packageIds.deepbookPackageId}::debate::Debate',
      ],
    );
  }
}
