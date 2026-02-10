import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

DeepBookConfig _config() {
  return DeepBookConfig(
    address: '0x1',
    balanceManagers: const {'m1': BalanceManager(address: '0x2')},
    marginManagers: const {'mm1': MarginManager(address: '0x3', poolKey: 'DEEP_SUI')},
  );
}

Map<String, dynamic> _lastCall(Transaction tx) {
  return tx.commands.last['MoveCall'] as Map<String, dynamic>;
}

String _target(Map<String, dynamic> call) {
  return '${call['package']}::${call['module']}::${call['function']}';
}

List<int> _argBytes(Transaction tx, dynamic arg) {
  if (arg is Map<String, dynamic> && arg['\$kind'] == 'Input') {
    final idx = arg['Input'] as int;
    final input = tx.inputs[idx];
    final pure = input['Pure'];
    if (pure is Map<String, dynamic> && pure['bytes'] is String) {
      return base64Decode(pure['bytes'] as String);
    }
  }
  throw StateError('argument is not pure bytes');
}

void main() {
  test('deepbook place limit order', () {
    final cfg = _config();
    final bm = BalanceManagerContract(cfg);
    final c = DeepBookContract(cfg, bm);
    final tx = Transaction();

    c.placeLimitOrder(
      tx,
      const PlaceLimitOrderParams(
        poolKey: 'DEEP_SUI',
        balanceManagerKey: 'm1',
        clientOrderId: '42',
        price: 1.5,
        quantity: 2,
        isBid: true,
      ),
    );

    final call = _lastCall(tx);
    expect(_target(call), contains('::pool::place_limit_order'));
    final expiration = BcsReader(_argBytes(tx, call['arguments'][10])).readU64();
    expect(expiration, maxTimestamp);
  });

  test('deepbook cancel orders u128 vec', () {
    final cfg = _config();
    final bm = BalanceManagerContract(cfg);
    final c = DeepBookContract(cfg, bm);
    final tx = Transaction();

    c.cancelOrders(tx, 'DEEP_SUI', 'm1', ['1', '2']);

    final call = _lastCall(tx);
    expect(_target(call), contains('::pool::cancel_orders'));
    final vec = _argBytes(tx, call['arguments'][3]);
    expect(vec[0], 2);
  });

  test('governance vote', () {
    final cfg = _config();
    final bm = BalanceManagerContract(cfg);
    final c = GovernanceContract(cfg, bm);
    final tx = Transaction();

    c.vote(tx, 'DEEP_SUI', 'm1', '7');
    expect(_target(_lastCall(tx)), contains('::pool::vote'));
  });

  test('margin manager order details two calls', () {
    final cfg = _config();
    final c = MarginManagerContract(cfg);
    final tx = Transaction();

    c.getMarginAccountOrderDetails(tx, 'mm1');

    expect(tx.commands.length, 2);
    expect(_target(tx.commands[0]['MoveCall'] as Map<String, dynamic>), contains('::margin_manager::balance_manager'));
    expect(_target(tx.commands[1]['MoveCall'] as Map<String, dynamic>), contains('::pool::get_account_order_details'));
  });
}
